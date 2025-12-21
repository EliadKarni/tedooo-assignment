package com.example.tedoooassigment

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.example.tedoooassigment.ui.feed.FeedFragment

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        if (savedInstanceState == null) {
            supportFragmentManager.beginTransaction()
                .replace(R.id.container, FeedFragment())
                .commitNow()
        }
    }
}
