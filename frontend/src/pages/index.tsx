import React, { useState } from "react";
import {
    ThemeProvider,
    CssBaseline,
    Container,
    Card,
    CardContent,
    Typography,
    Button,
    Dialog,
    DialogTitle,
    DialogContent,
    DialogActions,
    CardActions
} from "@mui/material";
import theme from "@/theme";

export default function HomePage() {
    const [openDialog, setOpenDialog] = useState(false);

    const handleDialogOpen = () => {
        setOpenDialog(true);
    };

    const handleDialogClose = () => {
        setOpenDialog(false);
    };

    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <Container
                sx={{
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                height: "100vh",
                }}
            >
                <Card sx={{ minWidth: 300, padding: 3 }}>
                    <CardContent>
                        <Typography variant="h5" component="div" sx={{ marginBottom: 2 }}>
                            Welcome to User Auth Template
                        </Typography>
                        <Typography variant="body2" color="text.secondary">
                            A fully made template for user authentaction.
                        </Typography>
                    </CardContent>
                    <CardActions>
                        <Button
                        href="/login"
                        variant="contained"
                        color="primary"
                        >
                            Log In
                        </Button>
                        <Button
                        href="/register"
                        variant="outlined"
                        color="primary"
                        sx={{ marginLeft: 1 }}
                        >
                            Register
                        </Button>
                        <Button
                        variant="outlined"
                        color="secondary"
                        onClick={handleDialogOpen}
                        sx={{ marginLeft: 1 }}
                        >
                            About
                        </Button>
                    </CardActions>
                </Card>
            </Container>

            <Dialog open={openDialog} onClose={handleDialogClose}>
                <DialogTitle>About</DialogTitle>
                <DialogContent>
                    <Typography variant="body2" color="text.secondary">
                        This user auth template was made using rocket.rs and sqlite on the backend, and nextjs + material-ui on the frontend. 
                        I made this as a project while bored, and to help people code similar things, please make a pull request if you have any improvements.
                        If you need anything help you can visit this discord here: https://discord.gg/Kvy9etJwvc and ask for help. The github is here: https://github.com/combogangreal/UserAuth
                    </Typography>
                </DialogContent>
                <DialogActions>
                    <Button onClick={handleDialogClose}>Close</Button>
                </DialogActions>
            </Dialog>
        </ThemeProvider>
    );
}
