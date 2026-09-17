package com.osxar.remit.file

import android.app.Activity
import android.net.Uri
import android.provider.OpenableColumns
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class DisplayNameArgs {
  lateinit var uri: String
}

@TauriPlugin
class FilePlugin(private val activity: Activity) : Plugin(activity) {
  @Command
  fun getDisplayName(invoke: Invoke) {
    try {
      val args = invoke.parseArgs(DisplayNameArgs::class.java)
      val uri = Uri.parse(args.uri)
      var name: String? = null
      val projection = arrayOf(OpenableColumns.DISPLAY_NAME)

      activity.contentResolver.query(uri, projection, null, null, null)?.use { cursor ->
        if (cursor.moveToFirst()) {
          val index = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
          if (index >= 0) {
            name = cursor.getString(index)
          }
        }
      }

      val result = JSObject()
      result.put("name", name ?: uri.lastPathSegment)
      invoke.resolve(result)
    } catch (error: Exception) {
      invoke.reject(error.message ?: "No se pudo obtener el nombre del archivo")
    }
  }
}
