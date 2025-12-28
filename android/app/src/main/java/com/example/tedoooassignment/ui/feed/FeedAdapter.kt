package com.example.tedoooassignment.ui.feed

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import coil.load
import com.example.tedoooassignment.R
import com.example.tedoooassignment.model.Product

class FeedAdapter(
    private val onClick: (Product) -> Unit
) : RecyclerView.Adapter<FeedAdapter.VH>() {

    private val items = mutableListOf<Product>()

    fun submitList(newItems: List<Product>) {
        items.clear()
        items.addAll(newItems)
        notifyDataSetChanged()
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): VH {
        val v = LayoutInflater.from(parent.context).inflate(R.layout.item_product, parent, false)
        return VH(v, onClick)
    }

    override fun onBindViewHolder(holder: VH, position: Int) = holder.bind(items[position])
    override fun getItemCount(): Int = items.size

    class VH(itemView: View, private val onClick: (Product) -> Unit) : RecyclerView.ViewHolder(itemView) {
        private val img: ImageView = itemView.findViewById(R.id.img)
        private val title: TextView = itemView.findViewById(R.id.title)
        private val desc: TextView = itemView.findViewById(R.id.desc)
        private val price: TextView = itemView.findViewById(R.id.price)

        fun bind(p: Product) {
            title.text = p.title
            desc.text = p.description ?: ""
            price.text = p.price
            img.load(p.imageUrl)
            itemView.setOnClickListener { onClick(p) }
        }
    }
}
