import React, { useState } from 'react';
import { ThemeProvider, TextField, Button, Typography, Container, CssBaseline, Card, CardContent } from '@mui/material';
import theme from '@/theme';  
import api from '@/api';

export default function Login() {
    const [formData, setFormData] = useState({
        method: '',
        password: '',
    });

    const [error, setError] = useState<string | null>(null);

    const textFieldOverrides = {
        '& .MuiOutlinedInput-root': {
            '&.Mui-focused fieldset': {
                borderColor: theme.palette.primary.main,
            },
        },
    };

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        try {
            const response = await api.login(formData.method, formData.password);
            if (response.ok) {
                const jwt: string = (await response.text()).toString();
                localStorage.setItem('jwt', jwt);
                window.location.href = `/success?jwt=${jwt}`;
            } else {
                const responseData = await response.json();
                setError(responseData.error || 'Login failed');
            }
        } catch (error) {
            console.error('Error during login:', error);
            setError('An unexpected error occurred');
        }
    };

    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <Container component="main" maxWidth="xs" sx={{ marginTop: 8, display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
                <Card sx={{ boxShadow: 4, minWidth: 275, backgroundColor: theme.palette.background.default, color: theme.palette.text.primary }}>
                    <CardContent>
                        <Typography component="h1" variant="h5" sx={{ marginBottom: 3, textAlign: 'center' }}>
                            Login
                        </Typography>
                        {error && (
                            <Typography color="error" variant="body2" sx={{ marginBottom: 2, textAlign: 'center' }}>
                                {error}
                            </Typography>
                        )}
                        <form onSubmit={handleSubmit} style={{ width: '100%' }}>
                            <TextField
                                label="Method (Username/Email/Phone)"
                                type="text"
                                name="method"
                                value={formData.method}
                                onChange={handleChange}
                                fullWidth
                                margin="normal"
                                variant="outlined"
                                required
                                sx={{ marginBottom: 2, ...textFieldOverrides }}
                                InputLabelProps={{ style: { color: theme.palette.text.primary } }}
                            />
                            <TextField
                                label="Password"
                                type="password"
                                name="password"
                                value={formData.password}
                                onChange={handleChange}
                                fullWidth
                                margin="normal"
                                variant="outlined"
                                required
                                sx={{ marginBottom: 2, ...textFieldOverrides }}
                                InputLabelProps={{ style: { color: theme.palette.text.primary } }}
                            />
                            <Button type="submit" variant="contained" color="primary" fullWidth sx={{ marginBottom: 2 }}>
                                Login
                            </Button>
                            <Typography variant="body2" sx={{ marginBottom: 2, textAlign: 'center' }}>
                                Don't have an account?{' '}
                                <a href="/register" style={{ cursor: 'pointer', color: theme.palette.primary.main }}>
                                    Register
                                </a>
                            </Typography>
                        </form>
                    </CardContent>
                </Card>
            </Container>
        </ThemeProvider>
    );
};