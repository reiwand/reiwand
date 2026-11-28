package io.github.reiwand

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.pm.PackageInstaller
import androidx.core.content.IntentCompat
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.ListenableWorker
import androidx.work.PeriodicWorkRequest
import androidx.work.WorkManager
import androidx.work.Worker
import androidx.work.WorkerParameters
import java.io.IOException
import java.util.concurrent.TimeUnit
import kotlin.concurrent.thread

/** The daily check of the Android app's own releases, while the automatic switch in About is on. */
class AppCheck(context: Context, params: WorkerParameters) : Worker(context, params) {
    override fun doWork(): ListenableWorker.Result {
        if (!AppUpdate.auto(applicationContext) || !Net.open()) return ListenableWorker.Result.success()
        return try {
            val found = AppUpdate.check(applicationContext)
            AppUpdate.found.value = found
            if (found is Found.Ready) AppInstall.notify(applicationContext, found.version)
            ListenableWorker.Result.success()
        } catch (refused: UpdateRefused) {
            AppUpdate.found.value = Found.Failed(refused.why)
            ListenableWorker.Result.success()
        } catch (_: IOException) {
            ListenableWorker.Result.retry()
        }
    }

    companion object {
        private const val WORK = "app_update"

        fun schedule(context: Context) {
            val work = WorkManager.getInstance(context)
            if (!AppUpdate.auto(context)) return work.cancelUniqueWork(WORK).let { }
            val request = PeriodicWorkRequest.Builder(AppCheck::class.java, 1, TimeUnit.DAYS).setConstraints(Network.constraints()).build()
            work.enqueueUniquePeriodicWork(WORK, ExistingPeriodicWorkPolicy.UPDATE, request)
        }
    }
}

/** Hands the checked APK to the system installer, which asks the person before replacing the app. */
object AppInstall {
    const val ACTION = "io.github.reiwand.INSTALL_UPDATE"
    private const val CHANNEL = "app_update"
    private const val ID = 4208

    fun notify(context: Context, version: String) {
        val notices = context.getSystemService(NotificationManager::class.java)
        if (!notices.areNotificationsEnabled()) return
        notices.createNotificationChannel(NotificationChannel(CHANNEL, context.getString(R.string.app_update_channel), NotificationManager.IMPORTANCE_DEFAULT))
        val open = Intent(context, Main::class.java).setAction(ACTION).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_SINGLE_TOP)
        notices.notify(
            ID,
            Notification.Builder(context, CHANNEL)
                .setSmallIcon(android.R.drawable.stat_sys_download_done)
                .setContentTitle(context.getString(R.string.app_update_ready, version))
                .setContentText(context.getString(R.string.app_update_tap))
                .setContentIntent(PendingIntent.getActivity(context, ID, open, PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT))
                .setAutoCancel(true)
                .build(),
        )
    }

    /** Called by Main for every intent it gets: only the notification's own action installs. */
    fun opened(context: Context, intent: Intent?) {
        if (intent?.action == ACTION) start(context)
    }

    fun start(context: Context) = thread(isDaemon = true) {
        context.getSystemService(NotificationManager::class.java).cancel(ID)
        val file = AppUpdate.apk(context).takeIf { AppUpdate.ready(context) != null }
        if (file == null) {
            AppUpdate.found.value = Found.Failed(R.string.app_update_gone)
            return@thread
        }
        runCatching {
            val installer = context.packageManager.packageInstaller
            val params = PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL).apply {
                setAppPackageName(context.packageName)
                setRequireUserAction(PackageInstaller.SessionParams.USER_ACTION_NOT_REQUIRED)
            }
            val id = installer.createSession(params)
            installer.openSession(id).use { session ->
                session.openWrite("reiwand.apk", 0, file.length()).use { out ->
                    file.inputStream().use { it.copyTo(out) }
                    session.fsync(out)
                }
                val back = Intent(context, Installed::class.java).setPackage(context.packageName)
                session.commit(PendingIntent.getBroadcast(context, id, back, PendingIntent.FLAG_MUTABLE or PendingIntent.FLAG_UPDATE_CURRENT).intentSender)
            }
        }.onFailure { AppUpdate.found.value = Found.Failed(R.string.app_update_failed) }
    }
}

/** The installer's answer: the system's confirmation to show, or why it did not install. */
class Installed : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        when (intent.getIntExtra(PackageInstaller.EXTRA_STATUS, PackageInstaller.STATUS_FAILURE)) {
            PackageInstaller.STATUS_PENDING_USER_ACTION ->
                IntentCompat.getParcelableExtra(intent, Intent.EXTRA_INTENT, Intent::class.java)?.let { context.startActivity(it.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)) }
            PackageInstaller.STATUS_SUCCESS -> AppUpdate.found.value = Found.Current
            PackageInstaller.STATUS_FAILURE_ABORTED -> AppUpdate.found.value = AppUpdate.ready(context)?.let { Found.Ready(it) } ?: Found.Idle
            else -> AppUpdate.found.value = Found.Failed(R.string.app_update_failed)
        }
    }
}
