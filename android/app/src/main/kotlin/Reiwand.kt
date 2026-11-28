package io.github.reiwand

import android.app.ActivityManager
import android.app.Application
import android.os.Build
import androidx.annotation.OptIn
import androidx.media3.common.util.Log
import androidx.media3.common.util.UnstableApi
import androidx.work.Configuration
import androidx.work.ExistingWorkPolicy
import kotlin.concurrent.thread

class Reiwand : Application(), Configuration.Provider {
    override val workManagerConfiguration: Configuration get() = Configuration.Builder().build()

    @OptIn(UnstableApi::class)
    override fun onCreate() {
        super.onCreate()
        Log.setLogLevel(Log.LOG_LEVEL_WARNING)
        saveUserRoots(noBackupFilesDir.resolve("roots.der"))
        Native.prepare(noBackupFilesDir.path)
        val small = getSystemService(ActivityManager::class.java)?.isLowRamDevice == true
        Native.device(small, Build.VERSION.MEDIA_PERFORMANCE_CLASS)
        thread(isDaemon = true) {
            Mirrors.watch(this, ExistingWorkPolicy.KEEP)
            Updates.schedule(this)
            ProviderWork.schedule(this)
            AppCheck.schedule(this)
            Converting.resume(this)
        }
    }
}
