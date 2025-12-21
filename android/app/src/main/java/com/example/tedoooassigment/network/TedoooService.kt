package com.example.tedoooassigment.network

import com.example.tedoooassigment.model.FeedResponse
import retrofit2.http.GET
import retrofit2.http.Query

interface TedoooService {
    @GET("hw/feed.json")
    suspend fun getFeed(
        @Query("skip") skip: String? = "0"
    ): FeedResponse
}
