package com.example.tedoooassignment.ui.feed

import androidx.lifecycle.*
import com.example.tedoooassignment.data.FeedRepository
import com.example.tedoooassignment.model.Product
import kotlinx.coroutines.launch

class FeedViewModel : ViewModel() {
    private val repo = FeedRepository()
    private val pageSize = 10

    private val _items = MutableLiveData<List<Product>>(emptyList())
    val items: LiveData<List<Product>> = _items

    private val _loading = MutableLiveData(false)
    val loading: LiveData<Boolean> = _loading

    private var nextCursor: String? = null

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
