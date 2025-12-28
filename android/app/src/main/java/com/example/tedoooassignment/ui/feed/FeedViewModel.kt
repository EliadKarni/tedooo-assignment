package com.example.tedoooassignment.ui.feed

import androidx.lifecycle.*
import com.example.tedoooassignment.data.FeedRepository
import com.example.tedoooassignment.model.Product
import kotlinx.coroutines.launch

/**
 * ViewModel for the Feed screen.
 * Manages the list of products, pagination, and loading state.
 */
class FeedViewModel : ViewModel() {
    private val repo = FeedRepository()
    private val pageSize = 10

    // LiveData holding the list of products to display
    private val _items = MutableLiveData<List<Product>>(emptyList())
    val items: LiveData<List<Product>> = _items

    // LiveData indicating the loading status
    private val _loading = MutableLiveData(false)
    val loading: LiveData<Boolean> = _loading

    // Cursor for the next page of results
    private var nextCursor: String? = null

    /**
     * Loads the initial page of products.
     * Clears existing data if needed (though currently implemented to just set the new items).
     */
    fun loadFirstPage() {
        if (_loading.value == true) return
        viewModelScope.launch {
            _loading.value = true
            runCatching { repo.fetchFirstPage(pageSize) }
                .onSuccess { page ->
                    _items.value = page.items
                    nextCursor = page.nextCursor
                }
            _loading.value = false
        }
    }

    /**
     * Loads the next page of products using the stored cursor.
     * Appends new items to the existing list.
     */
    fun loadNextPage() {
        if (_loading.value == true) return
        val cursor = nextCursor ?: return

        viewModelScope.launch {
            _loading.value = true
            runCatching { repo.fetchNextPage(pageSize, cursor) }
                .onSuccess { page ->
                    _items.value = _items.value.orEmpty() + page.items
                    nextCursor = page.nextCursor
                }
            _loading.value = false
        }
    }
}
