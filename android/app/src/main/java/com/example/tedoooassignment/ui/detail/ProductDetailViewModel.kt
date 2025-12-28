package com.example.tedoooassignment.ui.detail

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.tedoooassignment.data.FeedRepository
import com.example.tedoooassignment.model.Seller
import kotlinx.coroutines.launch

/**
 * ViewModel for the Product Detail screen.
 * Responsible for fetching additional details required for the product view, such as seller info.
 */
class ProductDetailViewModel : ViewModel() {
    private val repo = FeedRepository()

    // LiveData holding the seller information
    private val _seller = MutableLiveData<Seller>()
    val seller: LiveData<Seller> = _seller

    // LiveData indicating the loading status
    private val _loading = MutableLiveData(false)
    val loading: LiveData<Boolean> = _loading

    // LiveData holding any error that occurs during fetching
    private val _error = MutableLiveData<Throwable?>()
    val error: LiveData<Throwable?> = _error

    /**
     * Fetches seller information for a given seller ID.
     * Updates [seller], [loading], and [error] LiveData based on the result.
     *
     * @param sellerId The ID of the seller to fetch.
     */
    fun loadSellerInfo(sellerId: Long) {
        viewModelScope.launch {
            _loading.value = true
            _error.value = null
            runCatching {
                repo.getSeller(sellerId)
            }
            .onSuccess {
                _seller.value = it
            }
            .onFailure {
                _error.value = it
            }
            _loading.value = false
        }
    }
}
