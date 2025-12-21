package com.example.tedoooassigment.ui.feed

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import coil.load
import com.example.tedoooassigment.R
import com.example.tedoooassigment.model.FeedItem

class FeedAdapter(
    private var items: List<FeedItem> = emptyList(),
    private val onItemClick: (FeedItem) -> Unit
) : RecyclerView.Adapter<FeedAdapter.FeedViewHolder>() {

    fun updateData(newItems: List<FeedItem>) {
        items = newItems
        notifyDataSetChanged() // Ideally use DiffUtil
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): FeedViewHolder {
        val view = LayoutInflater.from(parent.context).inflate(R.layout.item_feed, parent, false)
        return FeedViewHolder(view, onItemClick)
    }

    override fun onBindViewHolder(holder: FeedViewHolder, position: Int) {
        holder.bind(items[position])
    }

    override fun getItemCount(): Int = items.size

    class FeedViewHolder(itemView: View, val onItemClick: (FeedItem) -> Unit) : RecyclerView.ViewHolder(itemView) {
        private val avatar: ImageView = itemView.findViewById(R.id.avatar)
        private val username: TextView = itemView.findViewById(R.id.username)
        private val shopName: TextView = itemView.findViewById(R.id.shopName)
        private val postImage: ImageView = itemView.findViewById(R.id.postImage)
        private val postText: TextView = itemView.findViewById(R.id.postText)
        private val likes: TextView = itemView.findViewById(R.id.likes)
        private val comments: TextView = itemView.findViewById(R.id.comments)

        fun bind(item: FeedItem) {
            username.text = item.username
            shopName.text = item.shopName ?: "Unknown Shop" // Handle null shopName if needed
            postText.text = item.text
            likes.text = "${item.likes} Likes"
            comments.text = "${item.comments} Comments"

            avatar.load(item.avatar) {
                placeholder(R.drawable.ic_launcher_foreground) // Use a proper placeholder
                error(R.drawable.ic_launcher_foreground)
                crossfade(true)
            }

            if (item.images.isNotEmpty()) {
                postImage.visibility = View.VISIBLE
                postImage.load(item.images[0]) {
                    placeholder(R.drawable.ic_launcher_foreground)
                    crossfade(true)
                }
            } else {
                postImage.visibility = View.GONE
            }

            itemView.setOnClickListener { onItemClick(item) }
        }
    }
}
