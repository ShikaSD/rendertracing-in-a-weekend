package com.shika.rayrender

import android.graphics.Bitmap
import android.graphics.Bitmap.Config.ARGB_8888
import android.graphics.Bitmap.createBitmap
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.renderscript.Allocation
import android.renderscript.RenderScript
import android.widget.ImageView
import androidx.core.view.doOnPreDraw

class MainActivity : AppCompatActivity() {
    private lateinit var resultView: ImageView

    private lateinit var resultBitmap: Bitmap
    private lateinit var alloc: Allocation

    private lateinit var script: ScriptC_ray

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        resultView = findViewById(R.id.result)

        resultView.doOnPreDraw {
            val rs = RenderScript.create(this)
            resultBitmap = createBitmap(resultView.measuredWidth, resultView.measuredHeight, ARGB_8888)
            alloc = Allocation.createFromBitmap(rs, resultBitmap)
            script = ScriptC_ray(rs)

            Thread(runnable).start()
        }
    }

    private val runnable = Runnable {
        script.forEach_hello_world(alloc, alloc)
        runOnUiThread {
            resultView.setImageBitmap(resultBitmap)
        }
    }
}
