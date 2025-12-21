package com.example.tedoooassigment.ui.feed

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.tedoooassigment.data.FeedRepository
import com.example.tedoooassigment.model.FeedItem
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

sealed class FeedUiState {
    object Loading : FeedUiState()
    data class Content(val items: List<FeedItem>, val hasMore: Boolean) : FeedUiState()
    data class Error(val message: String) : FeedUiState()
    object Empty : FeedUiState()
}

class FeedViewModel : ViewModel() {
    private val repository = FeedRepository()

    private val _uiState = MutableStateFlow<FeedUiState>(FeedUiState.Loading)
    val uiState: StateFlow<FeedUiState> = _uiState.asStateFlow()

    private val _items = mutableListOf<FeedItem>()
    private var nextCursor: String? = "0"
    private var hasMore = true
    private var isLoading = false

    init {
        loadFeed(isRefresh = true)
    }

    fun loadFeed(isRefresh: Boolean = false) {
        if (isLoading) return
        if (!isRefresh && !hasMore) return

        isLoading = true
        if (isRefresh) {
            _uiState.value = FeedUiState.Loading
            nextCursor = "0" // Reset cursor usually implies 0 or null depending on API. Using "0" as initial based on service.
            _items.clear()
            hasMore = true
        }

        viewModelScope.launch {
            try {
                // Determine cursor to send
                val cursorToSend = if (isRefresh) "0" else nextCursor
                
                val response = repository.getFeed(cursorToSend)
                
                nextCursor = response.nextCursor
                hasMore = response.hasMore
                
                _items.addAll(response.data)
                
                if (_items.isEmpty()) {
                    _uiState.value = FeedUiState.Empty
                } else {
                    _uiState.value = FeedUiState.Content(_items.toList(), hasMore)
                }
            } catch (e: Exception) {
                // Only show error state if we have no content to show, otherwise maybe show a toast (omitted for now)
                // or keep showing content but trigger a one-off event. 
                // For simplicity matching requirements: "UI states: loading, error, empty, content"
                if (_items.isEmpty()) {
                    _uiState.value = FeedUiState.Error(e.message ?: "Unknown error")
                } else {
                    // If we have items but pagination failed, we stay in Content state but maybe should notify UI?
                    // For now, let's keep it simple. The user can retry by scrolling or pulling.
                    // To handle retry on pagination failure properly, we might need a separate "PaginationError" state or event.
                    // But typically "Error" state replaces the screen content.
                    // I'll emit Content with current items.
                    _uiState.value = FeedUiState.Content(_items.toList(), hasMore)
                    // In a real app we'd send a side effect.
                }
            } finally {
                isLoading = false
            }
        }
    }
    
    fun refresh() {
        loadFeed(isRefresh = true)
    }

    fun loadNextPage() {
        loadFeed(isRefresh = false)
    }
}
