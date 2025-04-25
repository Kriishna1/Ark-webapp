// components/AddressList.jsx
import React, { useState, useEffect } from 'react';
import { getAddresses } from '../services/api';

const AddressList = ({ walletId, onSelectAddress }) => {
  const [addresses, setAddresses] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [selectedAddress, setSelectedAddress] = useState('');

  useEffect(() => {
    const fetchAddresses = async () => {
      if (!walletId) return;
      
      setLoading(true);
      try {
        const data = await getAddresses(walletId);
        setAddresses(data.addresses || []);
        setError('');
        
        // Select first address by default if available
        if (data.addresses && data.addresses.length > 0 && !selectedAddress) {
          handleSelectAddress(data.addresses[0].address);
        }
      } catch (err) {
        setError('Failed to load addresses');
        setAddresses([]);
      } finally {
        setLoading(false);
      }
    };

    fetchAddresses();
  }, [walletId]);

  const handleSelectAddress = (address) => {
    setSelectedAddress(address);
    if (onSelectAddress) {
      onSelectAddress(address);
    }
  };

  const copyToClipboard = (text) => {
    navigator.clipboard.writeText(text);
    // Could add a toast notification here
  };

  if (!walletId) {
    return <div className="text-gray-500">No wallet selected</div>;
  }

  if (loading) {
    return <div className="animate-pulse">Loading addresses...</div>;
  }

  if (error) {
    return <div className="text-red-500">{error}</div>;
  }

  return (
    <div className="border rounded-lg p-4 mb-6 bg-white shadow">
      <h2 className="text-xl font-bold mb-4">Addresses</h2>
      
      {addresses.length === 0 ? (
        <p className="text-gray-500">No addresses found for this wallet</p>
      ) : (
        <ul className="space-y-2">
          {addresses.map((addrInfo) => (
            <li 
              key={addrInfo.address}
              className={`p-3 border rounded cursor-pointer flex justify-between items-center
                ${selectedAddress === addrInfo.address ? 'bg-blue-50 border-blue-300' : 'hover:bg-gray-50'}`}
              onClick={() => handleSelectAddress(addrInfo.address)}
            >
              <div>
                <div className="font-mono text-sm truncate max-w-xs">{addrInfo.address}</div>
                <div className="text-xs text-gray-500">{addrInfo.path || 'External'}</div>
              </div>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  copyToClipboard(addrInfo.address);
                }}
                className="text-blue-600 hover:text-blue-800 text-sm"
              >
                Copy
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default AddressList;