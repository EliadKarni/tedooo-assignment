package com.example.tedoooassignment.ui.detail

import android.os.Build
import android.os.Bundle
import android.view.View
import android.widget.Button
import android.widget.ImageView
import android.widget.LinearLayout
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import coil.load
import coil.transform.CircleCropTransformation
import com.example.tedoooassignment.R
import com.example.tedoooassignment.model.Product

class ProductDetailFragment : Fragment(R.layout.fragment_product_detail) {

    private val viewModel: ProductDetailViewModel by viewModels()

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val product = getProductArg() ?: return

        view.findViewById<Button>(R.id.back_button).setOnClickListener {
            parentFragmentManager.popBackStack()
        }

        view.findViewById<ImageView>(R.id.img).load(product.imageUrl)
        view.findViewById<TextView>(R.id.title).text = product.title
        view.findViewById<TextView>(R.id.desc).text = product.description ?: ""
        view.findViewById<TextView>(R.id.price).text = product.price

        val sellerContainer = view.findViewById<LinearLayout>(R.id.seller_info_container)
        val sellerAvatar = view.findViewById<ImageView>(R.id.seller_avatar)
        val sellerName = view.findViewById<TextView>(R.id.seller_name)

        viewModel.seller.observe(viewLifecycleOwner) { seller ->
            sellerContainer.visibility = View.VISIBLE
            sellerName.text = seller.name
            sellerAvatar.load(seller.avatarUrl) {
                transformations(CircleCropTransformation())
            }
        }

        viewModel.loadSellerInfo(product.sellerId)
    }

    private fun getProductArg(): Product? {
        return if (Build.VERSION.SDK_INT >= 33) {
            requireArguments().getSerializable(ARG_PRODUCT, Product::class.java)
        } else {
            @Suppress("DEPRECATION")
            requireArguments().getSerializable(ARG_PRODUCT) as? Product
        }
    }

    companion object {
        private const val ARG_PRODUCT = "arg_product"

        fun newInstance(product: Product): ProductDetailFragment {
            return ProductDetailFragment().apply {
                arguments = Bundle().apply {
                    putSerializable(ARG_PRODUCT, product)
                }
            }
        }
    }
}
