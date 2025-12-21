package com.example.tedoooassigment.data

import com.example.tedoooassigment.model.FeedResponse
import com.example.tedoooassigment.network.NetworkClient

class FeedRepository {
    suspend fun getFeed(skip: String? = "0"): FeedResponse {
        return NetworkClient.tedoooService.getFeed(skip)
    }
}
