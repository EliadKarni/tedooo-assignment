package com.example.tedoooassignment.ui.product

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import com.example.tedoooassignment.R
import com.google.android.material.bottomsheet.BottomSheetBehavior
import com.google.android.material.bottomsheet.BottomSheetDialog
import com.google.android.material.bottomsheet.BottomSheetDialogFragment

class ProductDetailsSheet : BottomSheetDialogFragment() {

    companion object {
        private const val ARG_TITLE = "arg_title"
        private const val ARG_DESC = "arg_desc"
        private const val ARG_PRICE = "arg_price"
        private const val ARG_IMG = "arg_img"

        /**
         * Builds a new bottom sheet instance with product data.
         */
        fun newInstance(
            title: String,
            desc: String,
            price: String,
            imgUrl: String?
        ): ProductDetailsSheet {
            return ProductDetailsSheet().apply {
                arguments = Bundle().apply {
                    putString(ARG_TITLE, title)
                    putString(ARG_DESC, desc)
                    putString(ARG_PRICE, price)
                    putString(ARG_IMG, imgUrl)
                }
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        isCancelable = true
    }

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val root = inflater.inflate(R.layout.sheet_product_details, container, false)

        val title = requireArguments().getString(ARG_TITLE).orEmpty()
        val desc = requireArguments().getString(ARG_DESC).orEmpty()
        val price = requireArguments().getString(ARG_PRICE).orEmpty()
        val imgUrl = requireArguments().getString(ARG_IMG)

        root.findViewById<TextView>(R.id.title).text = title
        root.findViewById<TextView>(R.id.desc).text = desc
        root.findViewById<TextView>(R.id.price).text = price

        val img = root.findViewById<ImageView>(R.id.img)
        // TODO: load image with Coil/Glide if you use one:
        // img.load(imgUrl)

        return root
    }

    override fun onStart() {
        super.onStart()
        (dialog as? BottomSheetDialog)?.let { bsDialog ->
            val sheet = bsDialog.findViewById<View>(com.google.android.material.R.id.design_bottom_sheet)
            sheet?.let {
                val behavior = BottomSheetBehavior.from(it)
                behavior.state = BottomSheetBehavior.STATE_EXPANDED
                behavior.skipCollapsed = true
            }
            bsDialog.setCanceledOnTouchOutside(true)
        }
    }
}
