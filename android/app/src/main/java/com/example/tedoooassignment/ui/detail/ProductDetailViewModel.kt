package com.example.tedoooassignment.ui.detail

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.tedoooassignment.data.FeedRepository
import com.example.tedoooassignment.model.Seller
import kotlinx.coroutines.launch

class ProductDetailViewModel : ViewModel() {
    private val repo = FeedRepository()

    private val _seller = MutableLiveData<Seller>()
    val seller: LiveData<Seller> = _seller

    private val _loading = MutableLiveData(false)
    val loading: LiveData<Boolean> = _loading

    private val _error = MutableLiveData<Throwable?>()
    val error: LiveData<Throwable?> = _error

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
