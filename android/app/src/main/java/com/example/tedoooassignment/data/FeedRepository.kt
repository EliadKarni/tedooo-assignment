package com.example.tedoooassignment.data

import com.example.tedoooassignment.model.ProductsPage
import com.example.tedoooassignment.model.Seller
import com.example.tedoooassignment.network.NetworkClient
import com.example.tedoooassignment.network.TedoooService

/**
 * Repository responsible for fetching data related to the feed and products.
 * acts as a single source of truth for data operations.
 */
class FeedRepository {

    private val api: TedoooService =
        NetworkClient.retrofit.create(TedoooService::class.java)

    /**
     * Fetches the first page of products.
     *
     * @param limit The number of items to fetch.
     * @return [ProductsPage] containing the first batch of products.
     */
    suspend fun fetchFirstPage(limit: Int): ProductsPage {
        return api.getProducts(limit = limit, cursor = null)
    }

    /**
     * Fetches the next page of products based on the provided cursor.
     *
     * @param limit The number of items to fetch.
     * @param cursor The pagination cursor returned from the previous request.
     * @return [ProductsPage] containing the next batch of products.
     */
    suspend fun fetchNextPage(limit: Int, cursor: String): ProductsPage {
        return api.getProducts(limit = limit, cursor = cursor)
    }

    /**
     * Retrieves detailed information about a seller.
     *
     * @param sellerId The unique ID of the seller.
     * @return [Seller] object with seller details.
     */
    suspend fun getSeller(sellerId: Long): Seller {
        return api.getSeller(sellerId)
    }
}
