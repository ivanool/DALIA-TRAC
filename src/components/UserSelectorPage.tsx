import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Usuario {
  id: number;
  nombre: string;
  email?: string | null;
}

interface UserSelectorPageProps {
  onUserSelected: (user: Usuario) => void;
}

const UserSelectorPage: React.FC<UserSelectorPageProps> = ({ onUserSelected }) => {
  const [users, setUsers] = useState<Usuario[]>([]);
  const [loading, setLoading] = useState(true);
  const [newUserName, setNewUserName] = useState('');
  const [newUserEmail, setNewUserEmail] = useState('');
  const [error, setError] = useState<string | null>(null);

  const fetchUsers = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<Usuario[]>('get_users');
      setUsers(result);
    } catch (e: any) {
      setError('Error al cargar usuarios');
    }
    setLoading(false);
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  const handleSelect = (user: Usuario) => {
    onUserSelected(user);
  };

  const handleAddUser = async () => {
    if (!newUserName.trim() || !newUserEmail.trim()) return;
    setError(null);
    try {
      const user = await invoke<Usuario>('create_user', { nombre: newUserName, email: newUserEmail });
      setNewUserName('');
      setNewUserEmail('');
      setUsers([...users, user]);
      onUserSelected(user);
    } catch (e: any) {
      setError('No se pudo crear el usuario');
    }
  };

  return (
    <div className="user-selector-page">
      <h2>Selecciona tu usuario</h2>
      {loading ? (
        <p>Cargando usuarios...</p>
      ) : (
        <ul className="user-list">
          {users.map((user) => (
            <li key={user.id}>
              <button onClick={() => handleSelect(user)}>{user.nombre}</button>
              {user.email && <span className="desc">{user.email}</span>}
            </li>
          ))}
        </ul>
      )}
      <div className="add-user-section">
        <input
          type="text"
          placeholder="Nuevo usuario"
          value={newUserName}
          onChange={(e) => setNewUserName(e.target.value)}
        />
        <input
          type="email"
          placeholder="Email"
          value={newUserEmail}
          onChange={(e) => setNewUserEmail(e.target.value)}
        />
        <button onClick={handleAddUser}>Agregar usuario</button>
      </div>
      {error && <div className="error">{error}</div>}
    </div>
  );
};

export default UserSelectorPage;
