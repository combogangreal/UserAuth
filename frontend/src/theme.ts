import { createTheme } from '@mui/material/styles';

const theme = createTheme({
    palette: {
        mode: 'dark',
        primary: {
            main: '#9575cd',
            dark: '#65499c',
            light: '#c7a4ff',
        },
        secondary: {
            main: '#90caf9',
            dark: '#5d99c6',
            light: '#aedfff',
        },
        background: {
            default: '#121212',
            paper: '#1e1e1e',
        },
        text: {
            primary: '#ffffff',
            secondary: '#bdbdbd',
        },
    },
    components: {
        MuiButton: {
            styleOverrides: {
                root: {
                    backgroundColor: '#9575cd',
                    color: '#ffffff',
                    '&:hover': {
                        backgroundColor: '#65499c',
                    },
                },
            },
        },
        MuiTextField: {
            styleOverrides: {
                root: {
                    backgroundColor: '#1e1e1e',
                    color: '#ffffff',
                },
            },
        },
        MuiAppBar: {
            styleOverrides: {
                root: {
                    backgroundColor: '#121212',
                },
            },
        },
        MuiPaper: {
            styleOverrides: {
                root: {
                    backgroundColor: '#1e1e1e',
                    boxShadow: '0px 10px 20px rgba(0, 0, 0, 0.2)',
                },
            },
        },
    },
});

export default theme;
