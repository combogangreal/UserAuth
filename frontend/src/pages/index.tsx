import React, { useState } from "react";
import {
  ThemeProvider,
  Card,
  CardContent,
  Typography,
  Button,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogContentText,
  DialogActions,
} from "@mui/material";
import theme from "@/theme";

const HomePage = () => {
    const [openDialog, setOpenDialog] = useState(false);

    const handleOpenDialog = () => {
        setOpenDialog(true);
    };

    const handleCloseDialog = () => {
        setOpenDialog(false);
    };

    const handleLogin = () => {
        window.location.href = "/login";
    };

    const handleSignUp = () => {
        window.location.href = "/register";
    };

    return (
        <ThemeProvider theme={theme}>
            <div style={{ display: "flex", justifyContent: "center", alignItems: "center", height: "100vh" }}>
                <Card style={{ background: "#444", color: "#fff", outline: "none" }}>
                    <CardContent>
                        <Typography variant="h5" component="div" gutterBottom>
                            User Auth Template
                        </Typography>
                        <Typography variant="body2" color="text.secondary">
                            A user authentication template built with rust and nextjs
                        </Typography>
                        <div style={{ marginTop: "20px" }}>
                            <Button variant="contained" color="primary" onClick={handleLogin}>
                                Login
                            </Button>
                            <Button variant="contained" color="primary" style={{ marginLeft: "10px" }} onClick={handleSignUp}>
                                Sign Up
                            </Button>
                            <Button variant="contained" color="primary" style={{ marginLeft: "10px" }} onClick={handleOpenDialog}>
                                About
                            </Button>
                        </div>
                    </CardContent>
                </Card>

                <Dialog open={openDialog} onClose={handleCloseDialog}>
                    <DialogTitle>About Us</DialogTitle>
                    <DialogContent>
                        <DialogContentText>
                            This user authentication template was built using rocket.rs + sqlite on the backend for storing and creating users, and nextjs + material-ui for the frontend.
                            This was created by ComboGang, who you can contact through discord at https://discord.gg/2JgGJztY, all the code is also open source at https://github.com/combogangreal/UserAuth
                            If you wish to learn more, or need help, just ping me through the discord server or make a ticket.
                        </DialogContentText>
                    </DialogContent>
                    <DialogActions>
                        <Button onClick={handleCloseDialog} color="primary">
                            Close
                        </Button>
                    </DialogActions>
                </Dialog>
            </div>
        </ThemeProvider>
    );
};

export default HomePage;
