// components/BalanceDisplay.jsx
import React, { useState, useEffect } from 'react';
import { getBalance } from '../services/api';

const BalanceDisplay = ({ walletId }) => {
  const [balanceData, setBalanceData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [refreshing, setRefreshing] = useState(false);

  const fetchBalance = async () => {
    if (!walletId) return;
    
    setLoading(true);
    try {
      const data = await getBalance(walletId);
      setBalanceData(data);
      setError('');
    } catch (err) {
      setError('Failed to load balance');
      setBalanceData(null);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchBalance();
  }, [walletId]);

  const handleRefresh = async () => {
    setRefreshing(true);
    await fetchBalance();
    setRefreshing(false);
  };

  const formatCurrency = (amount, currency) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency || 'USD',
      minimumFractionDigits: 2,
      maximumFractionDigits: 8,
    }).format(amount);
  };

  if (!walletId) {
    return <div className="text-gray-500">No wallet selected</div>;
  }

  if (loading && !refreshing) {
    return <div className="animate-pulse">Loading balance...</div>;
  }

  if (error && !balanceData) {
    return <div className="text-red-500">{error}</div>;
  }

  return (
    <div className="border rounded-lg p-4 mb-6 bg-white shadow">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-bold">Wallet Balance</h2>
        <button
          onClick={handleRefresh}
          disabled={refreshing}
          className="text-blue-600 hover:text-blue-800 disabled:text-gray-400"
        >
          {refreshing ? 'Refreshing...' : 'Refresh'}
        </button>
      </div>
      
      {balanceData && (
        <div className="space-y-4">
          <div className="text-center">
            <div className="text-3xl font-bold">
              {formatCurrency(balanceData.totalBalance, balanceData.currency)}
            </div>
            <div className="text-gray-500">Total Balance</div>
          </div>
          
          <div className="grid grid-cols-2 gap-4 mt-4">
            <div className="text-center p-3 bg-gray-50 rounded">
              <div className="text-lg font-semibold text-green-600">
                {formatCurrency(balanceData.availableBalance, balanceData.currency)}
              </div>
              <div className="text-sm text-gray-500">Available</div>
            </div>
            
            <div className="text-center p-3 bg-gray-50 rounded">
              <div className="text-lg font-semibold text-yellow-600">
                {formatCurrency(balanceData.pendingBalance, balanceData.currency)}
              </div>
              <div className="text-sm text-gray-500">Pending</div>
            </div>
          </div>
          
          {balanceData.lastUpdated && (
            <div className="text-xs text-right text-gray-500">
              Last updated: {new Date(balanceData.lastUpdated).toLocaleString()}
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default BalanceDisplay;