package io.github.reiwand

import android.content.Context
import androidx.work.Constraints
import androidx.work.ExistingWorkPolicy
import androidx.work.NetworkType

object Network {
    const val SETTING = "sync_network"

    fun changed(context: Context) {
        Updates.schedule(context)
        ProviderWork.schedule(context)
        Mirrors.watch(context, ExistingWorkPolicy.KEEP)
    }

    fun constraints() = Constraints.Builder()
        .setRequiredNetworkType(if (Native.setting(SETTING, null) == "any") NetworkType.CONNECTED else NetworkType.UNMETERED)
        .setRequiresBatteryNotLow(true)
        .build()
}
