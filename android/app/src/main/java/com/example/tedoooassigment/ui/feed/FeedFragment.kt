package com.example.tedoooassigment.ui.feed

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.ProgressBar
import android.widget.TextView
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import androidx.swiperefreshlayout.widget.SwipeRefreshLayout
import com.example.tedoooassigment.R
import com.example.tedoooassigment.model.FeedItem
import com.example.tedoooassigment.ui.detail.ProductDetailFragment
import kotlinx.coroutines.launch

class FeedFragment : Fragment() {

    private val viewModel: FeedViewModel by viewModels()
    private lateinit var adapter: FeedAdapter
    
    private lateinit var swipeRefreshLayout: SwipeRefreshLayout
    private lateinit var recyclerView: RecyclerView
    private lateinit var progressBar: ProgressBar
    private lateinit var errorText: TextView
    private lateinit var retryButton: Button

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        return inflater.inflate(R.layout.fragment_feed, container, false)
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        swipeRefreshLayout = view.findViewById(R.id.swipeRefreshLayout)
        recyclerView = view.findViewById(R.id.recyclerView)
        progressBar = view.findViewById(R.id.progressBar)
        errorText = view.findViewById(R.id.errorText)
        retryButton = view.findViewById(R.id.retryButton)

        setupRecyclerView()
        setupObservers()
        setupListeners()
    }

    private fun setupRecyclerView() {
        adapter = FeedAdapter { feedItem ->
            parentFragmentManager.beginTransaction()
                .replace(R.id.container, ProductDetailFragment.newInstance(feedItem))
                .addToBackStack(null)
                .commit()
        }
        recyclerView.adapter = adapter
        
        val layoutManager = recyclerView.layoutManager as LinearLayoutManager
        recyclerView.addOnScrollListener(object : RecyclerView.OnScrollListener() {
            override fun onScrolled(recyclerView: RecyclerView, dx: Int, dy: Int) {
                super.onScrolled(recyclerView, dx, dy)
                
                val visibleItemCount = layoutManager.childCount
                val totalItemCount = layoutManager.itemCount
                val firstVisibleItemPosition = layoutManager.findFirstVisibleItemPosition()

                if ((visibleItemCount + firstVisibleItemPosition) >= totalItemCount
                    && firstVisibleItemPosition >= 0
                ) {
                    viewModel.loadNextPage()
                }
            }
        })
    }

    private fun setupListeners() {
        swipeRefreshLayout.setOnRefreshListener {
            viewModel.refresh()
        }
        
        retryButton.setOnClickListener {
            viewModel.loadFeed(isRefresh = true)
        }
    }

    private fun setupObservers() {
        viewLifecycleOwner.lifecycleScope.launch {
            viewLifecycleOwner.repeatOnLifecycle(Lifecycle.State.STARTED) {
                viewModel.uiState.collect { state ->
                    when (state) {
                        is FeedUiState.Loading -> {
                            // Only show full screen loader if we have no data yet?
                            // SwipeRefreshLayout has its own indicator.
                            if (!swipeRefreshLayout.isRefreshing) {
                                progressBar.visibility = View.VISIBLE
                            }
                            errorText.visibility = View.GONE
                            retryButton.visibility = View.GONE
                            recyclerView.visibility = View.GONE // Hide list on initial load
                        }
                        is FeedUiState.Content -> {
                            progressBar.visibility = View.GONE
                            swipeRefreshLayout.isRefreshing = false
                            errorText.visibility = View.GONE
                            retryButton.visibility = View.GONE
                            recyclerView.visibility = View.VISIBLE
                            adapter.updateData(state.items)
                        }
                        is FeedUiState.Error -> {
                            progressBar.visibility = View.GONE
                            swipeRefreshLayout.isRefreshing = false
                            errorText.visibility = View.VISIBLE
                            errorText.text = state.message
                            retryButton.visibility = View.VISIBLE
                            recyclerView.visibility = View.GONE
                        }
                        is FeedUiState.Empty -> {
                            progressBar.visibility = View.GONE
                            swipeRefreshLayout.isRefreshing = false
                            // Maybe show empty view
                        }
                    }
                }
            }
        }
    }
}
