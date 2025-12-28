package com.example.tedoooassignment.network

import com.example.tedoooassignment.model.ProductsPage
import com.example.tedoooassignment.model.Seller
import retrofit2.http.GET
import retrofit2.http.Path
import retrofit2.http.Query

/**
 * Retrofit service interface for the Tedooo API.
 */
interface TedoooService {

    /**
     * Fetches a paginated list of products.
     *
     * @param limit The maximum number of items to return in one page.
     * @param cursor The cursor for pagination. Pass null for the first page.
     * @return A [ProductsPage] object containing the list of products and the next cursor.
     */
    @GET("products")
    suspend fun getProducts(
        @Query("limit") limit: Int,
        @Query("cursor") cursor: String? = null
    ): ProductsPage

    /**
     * Fetches details for a specific seller.
     *
     * @param id The unique identifier of the seller.
     * @return A [Seller] object containing the seller's details.
     */
    @GET("seller/{id}")
    suspend fun getSeller(
        @Path("id") id: Long
    ): Seller
}
