package io.github.reiwand

import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

object Vault {
    private const val ALIAS = "reiwand-account"
    private const val CIPHER = "AES/GCM/NoPadding"
    private const val IV = 12

    private fun key(): SecretKey {
        val store = KeyStore.getInstance("AndroidKeyStore").apply { load(null) }
        return store.getKey(ALIAS, null) as SecretKey? ?: KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, "AndroidKeyStore").run {
            val purposes = KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
            init(
                KeyGenParameterSpec.Builder(ALIAS, purposes)
                    .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                    .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                    .setKeySize(256)
                    .build(),
            )
            generateKey()
        }
    }

    @JvmStatic
    fun seal(plain: ByteArray): ByteArray = Cipher.getInstance(CIPHER).run {
        init(Cipher.ENCRYPT_MODE, key())
        iv + doFinal(plain)
    }

    @JvmStatic
    fun open(sealed: ByteArray): ByteArray = Cipher.getInstance(CIPHER).run {
        init(Cipher.DECRYPT_MODE, key(), GCMParameterSpec(128, sealed, 0, IV))
        doFinal(sealed, IV, sealed.size - IV)
    }
}
