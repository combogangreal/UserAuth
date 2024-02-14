'use client';
import { createTheme } from '@mui/material/styles';
import { red, grey } from '@mui/material/colors';

const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#556cd6', 
      light: '#8297ff',
      dark: '#2b3fa9',
      contrastText: '#fff',
    },
    secondary: {
      main: grey[700],  
      light: grey[600],
      dark: grey[800],
      contrastText: '#fff',
    },
    error: {
      main: red.A400,
    },
    background: {
      default: '#151A21', 
      paper: '#202731',   
    }
  },
  typography: {
    fontFamily: [
      'Roboto',
      '-apple-system',
      'BlinkMacSystemFont',
      '"Segoe UI"',
      '"Helvetica Neue"',
      'Arial',
      'sans-serif',
      '"Apple Color Emoji"',
      '"Segoe UI Emoji"',
      '"Segoe UI Symbol"',
    ].join(','),
  },
  components: {
    MuiButton: {
      defaultProps: {
        disableElevation: true, 
      },
      variants: [
        {
          props: { variant: 'contained' },
          style: {
            textTransform: 'none', 
          },
        },
      ],
    },
  },
});

export default theme;