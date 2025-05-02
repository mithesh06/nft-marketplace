// pages/_app.tsx
import type { AppProps } from 'next/app';
import { ClientWalletProvider } from '../components/ClientWalletProvider';
import '../styles/globals.css';
import '@solana/wallet-adapter-react-ui/styles.css';

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ClientWalletProvider>
      <Component {...pageProps} />
    </ClientWalletProvider>
  );
}
