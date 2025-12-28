package com.example.tedoooassignment.data

import com.example.tedoooassignment.model.ProductsPage
import com.example.tedoooassignment.model.Seller
import com.example.tedoooassignment.network.NetworkClient
import com.example.tedoooassignment.network.TedoooService

class FeedRepository {

    private val api: TedoooService =
        NetworkClient.retrofit.create(TedoooService::class.java)

    suspend fun fetchFirstPage(limit: Int): ProductsPage {
        return api.getProducts(limit = limit, cursor = null)
    }

    suspend fun fetchNextPage(limit: Int, cursor: String): ProductsPage {
        return api.getProducts(limit = limit, cursor = cursor)
    }

    suspend fun getSeller(sellerId: Long): Seller {
        return api.getSeller(sellerId)
    }
}
