package com.example.tedoooassignment.model

import com.google.gson.annotations.SerializedName
import java.io.Serializable

/**
 * Represents a paginated response containing a list of products.
 *
 * @property items The list of products in the current page.
 * @property nextCursor The cursor to be used for fetching the next page of results, or null if there are no more pages.
 */
data class ProductsPage(
    val items: List<Product>,
    @SerializedName("next_cursor") val nextCursor: String?
)

/**
 * Represents a single product item in the feed.
 *
 * @property id The unique identifier of the product.
 * @property title The title of the product.
 * @property description A brief description of the product.
 * @property price The price of the product as a formatted string (e.g., "$10.00").
 * @property imageUrl The URL of the product image.
 * @property createdAt The timestamp when the product was created.
 * @property sellerId The ID of the seller who listed this product.
 */
data class Product(
    val id: Long,
    val title: String,
    val description: String?,
    val price: String,
    @SerializedName("image_url") val imageUrl: String?,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("seller_id") val sellerId: Long
) : Serializable

/**
 * Represents a seller of a product.
 *
 * @property id The unique identifier of the seller.
 * @property name The name of the seller.
 * @property avatarUrl The URL of the seller's avatar image.
 */
data class Seller(
    val id: Long,
    val name: String,
    @SerializedName("avatar_url") val avatarUrl: String?
)
