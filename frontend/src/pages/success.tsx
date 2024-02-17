import React, { useEffect, useState } from 'react';
import { Typography, Container, CssBaseline, Card, CardContent, ThemeProvider, createTheme } from '@mui/material';
import theme from '@/theme';  
import api from '@/api';

interface DecodedJWT {
    sub: string;
    iat: number;
    username: string;
    email: string;
    phone: string;
    method: string;
    password: string;
    exp: number;
    success: boolean;
    error?: string;
}

function timestampToTime(timestamp: number): string {
    const date = new Date(timestamp * 1000); 

    const hours = date.getUTCHours().toString().padStart(2, '0');
    const minutes = date.getUTCMinutes().toString().padStart(2, '0');
    const seconds = date.getUTCSeconds().toString().padStart(2, '0');

    return `${hours}:${minutes}:${seconds}`;
}

function calculateRemainingTime(decodedPayload: DecodedJWT): number {
    const currentTime = Math.floor(Date.now() / 1000);
    const remainingTime = decodedPayload.exp - currentTime;
    return Math.max(remainingTime, 0);
}

function startLogoutTimer(decodedPayload: DecodedJWT): void {
    const remainingTime = calculateRemainingTime(decodedPayload);

    if (remainingTime > 0) {
        setTimeout(() => {
            api.logout(decodedPayload.email);
            window.location.href = '/';
            console.log('Session timeout. Logging out...');
        }, remainingTime * 1000);
    }
}

export default function Success() {
    const [decodedJWT, setDecodedJWT] = useState<DecodedJWT | null>(null);

    useEffect(() => {
        const decodeJWTFromAPI = async () => {
            try {
                const urlParams = new URLSearchParams(window.location.search);
                const jwt = urlParams.get('jwt');

                if (jwt) {
                    const response = await api.decodeJWT(jwt); 
                    if (response.ok) {
                        const responseData = await response.json();
                        console.log('JWT decoded successfully. Server response:', responseData);
                        startLogoutTimer(responseData);
                        setDecodedJWT(responseData);
                    } else {
                        const errorResponse = await response.json();
                        console.error('JWT decoding failed. Server response:', errorResponse);
                    }
                }
            } catch (error) {
                console.error('JWT decoding failed:', error);
            }
        };

        decodeJWTFromAPI();
    }, []); 

    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <Container component="main" maxWidth="xs" sx={{ marginTop: 8, display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
                <Card sx={{ boxShadow: 4, minWidth: 275, backgroundColor: theme.palette.background.default, color: theme.palette.text.primary }}>
                    <CardContent>
                        <Typography component="h1" variant="h5" sx={{ marginBottom: 3, textAlign: 'center' }}>
                            User Info
                        </Typography>
                        {decodedJWT && decodedJWT.success ? (
                            <div>
                                <Typography variant="body1" sx={{ marginBottom: 2, textAlign: 'center' }}>
                                    Welcome, {decodedJWT.username}!
                                </Typography>
                                <Typography variant="body2" sx={{ textAlign: 'center' }}>
                                    Your user ID: {decodedJWT.sub}
                                </Typography>
                                <Typography variant="body2" sx={{ textAlign: 'center' }}>
                                    Your password: {decodedJWT.password}
                                </Typography>
                                <Typography variant="body2" sx={{ textAlign: 'center' }}>
                                    Your email: {decodedJWT.email}
                                </Typography>
                                <Typography variant="body2" sx={{ textAlign: 'center' }}>
                                    Your phone: {decodedJWT.phone}
                                </Typography>
                                <Typography variant="body2" sx={{ textAlign: 'center' }}>
                                    Time until JWT expires: {timestampToTime(calculateRemainingTime(decodedJWT))}
                                </Typography>
                            </div>
                        ) : (
                            <Typography variant="body2" color="error" sx={{ textAlign: 'center' }}>
                                Providing user info failed. {decodedJWT?.error}
                            </Typography>
                        )}
                    </CardContent>
                </Card>
            </Container>
        </ThemeProvider>
    );
}
