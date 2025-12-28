package com.example.tedoooassignment.ui.feed

import android.os.Bundle
import android.view.View
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.example.tedoooassignment.R
import com.example.tedoooassignment.data.FeedRepository
import com.example.tedoooassignment.ui.detail.ProductDetailFragment
import androidx.swiperefreshlayout.widget.SwipeRefreshLayout

class FeedFragment : Fragment(R.layout.fragment_feed) {

    private val vm: FeedViewModel by viewModels {
        FeedViewModelFactory(FeedRepository())
    }
    private lateinit var adapter: FeedAdapter

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val swipe = view.findViewById<SwipeRefreshLayout>(R.id.swipeRefresh)
        val rv = view.findViewById<RecyclerView>(R.id.recyclerView)
        val errorText = view.findViewById<TextView>(R.id.errorText)

        adapter = FeedAdapter { product ->
            parentFragmentManager.beginTransaction()
                .replace(R.id.container, ProductDetailFragment.newInstance(product))
                .addToBackStack(null)
                .commit()
        }

        rv.layoutManager = LinearLayoutManager(requireContext())
        rv.adapter = adapter

        vm.items.observe(viewLifecycleOwner) {
            adapter.submitList(it)
            if (it.isNotEmpty()) {
                rv.visibility = View.VISIBLE
                errorText.visibility = View.GONE
            }
        }
        vm.loading.observe(viewLifecycleOwner) { swipe.isRefreshing = it == true }
        vm.error.observe(viewLifecycleOwner) { isError ->
            if (isError && vm.items.value.isNullOrEmpty()) {
                rv.visibility = View.GONE
                errorText.visibility = View.VISIBLE
            } else {
                errorText.visibility = View.GONE
            }
        }

        swipe.setOnRefreshListener { vm.loadFeed(forceRefresh = true) }

        rv.addOnScrollListener(object : RecyclerView.OnScrollListener() {
            override fun onScrolled(recyclerView: RecyclerView, dx: Int, dy: Int) {
                if (dy <= 0) return
                val lm = recyclerView.layoutManager as LinearLayoutManager
                val lastVisible = lm.findLastVisibleItemPosition()
                if (lastVisible >= lm.itemCount - 3) vm.loadNextPage()
            }
        })

        // Initial load (will only fetch if list is empty)
        vm.loadFeed(forceRefresh = false)
    }
}
