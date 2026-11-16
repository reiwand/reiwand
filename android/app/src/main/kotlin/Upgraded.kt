package io.github.reiwand

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

class Upgraded : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        Native.upgrade(context.noBackupFilesDir.path)
    }
}
