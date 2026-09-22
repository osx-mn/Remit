package com.osxar.remit.file

import android.app.Activity
import android.content.ContentValues
import android.net.Uri
import android.os.Environment
import android.provider.MediaStore
import android.provider.OpenableColumns
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.io.File

// Argumentos para obtener el nombre real de un archivo desde su URI content://
@InvokeArg
class DisplayNameArgs {
  lateinit var uri: String
}

// Argumentos para copiar un archivo privado hacia la carpeta pública Documentos
@InvokeArg
class SaveToDocumentsArgs {
  lateinit var filePath: String
  lateinit var fileName: String
}

@TauriPlugin
class FilePlugin(private val activity: Activity) : Plugin(activity) {

  // Resuelve el nombre real de un archivo a partir de su URI (usado al enviar archivos por FTP)
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

  // Copia un archivo recibido por FTP (guardado en carpeta privada) hacia Documentos/Remit,
  // usando MediaStore.Files, ya que Documentos no tiene colección dedicada como Descargas
  @Command
  fun saveToDocuments(invoke: Invoke) {
    try {
      val args = invoke.parseArgs(SaveToDocumentsArgs::class.java)
      val resolver = activity.contentResolver

      // Define nombre y carpeta destino dentro de Documentos
      val values = ContentValues().apply {
        put(MediaStore.Files.FileColumns.DISPLAY_NAME, args.fileName)
        put(MediaStore.Files.FileColumns.MIME_TYPE, "application/octet-stream")
        put(MediaStore.Files.FileColumns.RELATIVE_PATH, Environment.DIRECTORY_DOCUMENTS + "/Remit")
      }

      // Crea el registro en la colección genérica de archivos y obtiene la URI donde escribir
      val uri = resolver.insert(MediaStore.Files.getContentUri("external"), values)
        ?: throw Exception("No se pudo crear el registro en MediaStore")

      // Copia el contenido del archivo privado hacia la URI pública
      resolver.openOutputStream(uri).use { out ->
        File(args.filePath).inputStream().use { input ->
          input.copyTo(out!!)
        }
      }

      invoke.resolve(JSObject())
    } catch (error: Exception) {
      invoke.reject(error.message ?: "Error guardando en Documentos")
    }
  }
}