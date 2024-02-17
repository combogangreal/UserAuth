import React, { useState } from 'react';
import { TextField, Button, Typography, Link, Container, CssBaseline, Card, CardContent, ThemeProvider, createTheme } from '@mui/material';
import theme from '@/theme';  // Adjust the path based on your project structure
import api from '@/api';  // Adjust the path based on your project structure

interface FormData {
    username: string;
    email: string;
    phone: string;
    password: string;
}

export default function Register() {
    const [formData, setFormData] = useState<FormData>({
        username: '',
        email: '',
        phone: '',
        password: '',
    });

    const [error, setError] = useState<string | null>(null);

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        try {
            const response = await api.register(formData.username, formData.email, formData.phone, formData.password);
            console.log(response);
            if (response.ok) {
                const responseData = await response;
                console.log(responseData);
                const jwt: string = (await responseData.text()).toString();
                localStorage.setItem('jwt', jwt);

                window.location.href = `/success?jwt=${jwt}`;
            } else {
                const responseText: string = await response.text();
                console.error('Registration failed. Server response:', responseText);
                setError(responseText || 'Registration failed');
            }
        } catch (error: any) {
            console.error('Error during registration:', error);
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
                            Register
                        </Typography>
                        {error && (
                            <Typography color="error" variant="body2" sx={{ marginBottom: 2, textAlign: 'center' }}>
                                {error}
                            </Typography>
                        )}
                        <form onSubmit={handleSubmit} style={{ width: '100%' }}>
                            <TextField
                                label="Username"
                                type="text"
                                name="username"
                                value={formData.username}
                                onChange={handleChange}
                                fullWidth
                                margin="normal"
                                variant="outlined"
                                required
                            />
                            <TextField
                                label="Email"
                                type="email"
                                name="email"
                                value={formData.email}
                                onChange={handleChange}
                                fullWidth
                                margin="normal"
                                variant="outlined"
                                required
                            />
                            <TextField
                                label="Phone"
                                type="text"
                                name="phone"
                                value={formData.phone}
                                onChange={handleChange}
                                fullWidth
                                margin="normal"
                                variant="outlined"
                                required
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
                            />
                            <Button type="submit" variant="contained" color="primary" fullWidth sx={{ marginBottom: 2 }}>
                                Register
                            </Button>
                            <Typography variant="body2" sx={{ marginBottom: 2, textAlign: 'center' }}>
                                Already have an account?{' '}
                                <Link href="/login" variant="body2" color="primary">
                                    Sign In
                                </Link>
                            </Typography>
                        </form>
                    </CardContent>
                </Card>
            </Container>
        </ThemeProvider>
    );
};
