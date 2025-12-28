package com.example.tedoooassignment

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.example.tedoooassignment.ui.feed.FeedFragment

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // Avoid re-adding the fragment on configuration changes
        if (savedInstanceState == null) {
            supportFragmentManager.beginTransaction()
                .replace(R.id.container, FeedFragment())
                .commit()
        }
    }
}
