package com.example.tedoooassignment.model

import com.google.gson.annotations.SerializedName
import java.io.Serializable

data class ProductsPage(
    val items: List<Product>,
    @SerializedName("next_cursor") val nextCursor: String?
)

data class Product(
    val id: Long,
    val title: String,
    val description: String?,
    val price: String,
    @SerializedName("image_url") val imageUrl: String?,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("seller_id") val sellerId: Long
) : Serializable

data class Seller(
    val id: Long,
    val name: String,
    @SerializedName("avatar_url") val avatarUrl: String?
)
