package com.example.tedoooassigment.model

import android.os.Parcelable
import com.google.gson.annotations.SerializedName
import kotlinx.parcelize.Parcelize

data class FeedResponse(
    @SerializedName("hasMore")
    val hasMore: Boolean,
    @SerializedName("nextCursor")
    val nextCursor: String?,
    @SerializedName("data")
    val data: List<FeedItem>
)

@Parcelize
data class FeedItem(
    @SerializedName("id")
    val id: String,
    @SerializedName("text")
    val text: String,
    @SerializedName("images")
    val images: List<String>,
    @SerializedName("userId")
    val userId: String,
    @SerializedName("username")
    val username: String,
    @SerializedName("avatar")
    val avatar: String?,
    @SerializedName("shopName")
    val shopName: String?,
    @SerializedName("likes")
    val likes: Int,
    @SerializedName("comments")
    val comments: Int,
    @SerializedName("date")
    val date: String
) : Parcelable
