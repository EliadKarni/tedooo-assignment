package com.example.tedoooassigment.ui.detail

import android.os.Build
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.fragment.app.Fragment
import coil.load
import com.example.tedoooassigment.R
import com.example.tedoooassigment.model.FeedItem

class ProductDetailFragment : Fragment() {

    private var feedItem: FeedItem? = null

    companion object {
        private const val ARG_FEED_ITEM = "arg_feed_item"

        fun newInstance(feedItem: FeedItem): ProductDetailFragment {
            val fragment = ProductDetailFragment()
            val args = Bundle()
            args.putParcelable(ARG_FEED_ITEM, feedItem)
            fragment.arguments = args
            return fragment
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        arguments?.let {
            feedItem = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                it.getParcelable(ARG_FEED_ITEM, FeedItem::class.java)
            } else {
                it.getParcelable(ARG_FEED_ITEM)
            }
        }
    }

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        return inflater.inflate(R.layout.fragment_product_detail, container, false)
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        
        feedItem?.let { item ->
            val avatar: ImageView = view.findViewById(R.id.detailAvatar)
            val username: TextView = view.findViewById(R.id.detailUsername)
            val shopName: TextView = view.findViewById(R.id.detailShopName)
            val postImage: ImageView = view.findViewById(R.id.detailPostImage)
            val postText: TextView = view.findViewById(R.id.detailPostText)
            val likes: TextView = view.findViewById(R.id.detailLikes)
            val comments: TextView = view.findViewById(R.id.detailComments)

            username.text = item.username
            shopName.text = item.shopName ?: "Unknown Shop"
            postText.text = item.text
            likes.text = "${item.likes} Likes"
            comments.text = "${item.comments} Comments"

            avatar.load(item.avatar) {
                placeholder(R.drawable.ic_launcher_foreground)
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
        }
    }
}
