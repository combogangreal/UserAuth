import { API_BASE_URL, SECRET_KEY } from './config';

const api = {
    post: async (endpoint: string, data: string) => {
        try {
            const response = await fetch(`${API_BASE_URL}/${endpoint}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                },
                body: data,
            });
    
            if (!response.ok) {
                throw new Error(`Request to ${endpoint} failed with status: ${response.status}`);
            }
            console.log(response);
            return await response;
        } catch (error) {
            console.error(`Request to ${endpoint} failed:`, error);
            throw error;
        }
    },
  
    register: async (username: string, email: string, phone: string, password: string) => {
        return api.post('register', `username=${username}&email=${email}&phone=${phone}&password=${password}&secret_key=${SECRET_KEY}`);
    },
  
    login: async (method: string, password: string) => {
        return api.post('login', `method=${method}&password=${password}&secret_key=${SECRET_KEY}`);
    },
  
    logout: async (method: string) => {
        return api.post('logout', `method=${method}&secret_key=${SECRET_KEY}`);
    },

    verifyJWT: async (token: string) => {
        return api.post('verifyjwt', `token=${token}&secret_key=${SECRET_KEY}`); 
    },
  
    decodeJWT: async (token: string) => {
        return api.post('decodejwt', `token=${token}&secret_key=${SECRET_KEY}`);
    },
};
  
export default api;