package com.example.tedoooassignment.network

import com.example.tedoooassignment.model.ProductsPage
import com.example.tedoooassignment.model.Seller
import retrofit2.http.GET
import retrofit2.http.Path
import retrofit2.http.Query

interface TedoooService {

    @GET("products")
    suspend fun getProducts(
        @Query("limit") limit: Int,
        @Query("cursor") cursor: String? = null
    ): ProductsPage

    @GET("seller/{id}")
    suspend fun getSeller(
        @Path("id") id: Long
    ): Seller
}
