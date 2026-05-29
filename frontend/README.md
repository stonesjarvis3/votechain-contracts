# VoteChain Frontend Components

This directory contains the frontend components for the VoteChain project.

## Components

### Navbar

A responsive navigation bar with a mobile hamburger menu and a network indicator.

- **Desktop View**: Horizontal list of navigation links and a network badge next to the logo.
- **Mobile View (< 768px)**: Hamburger icon that opens a slide-in side menu containing the network information and links.
- **Network Indicator**: 
  - Displays the current Stellar network.
  - Color-coded: **Mainnet** (Green), **Testnet** (Yellow), **Local** (Grey).
  - **Wallet Mismatch**: Shows a warning (⚠️) if the connected wallet's network doesn't match the application network.
- **Interactions**:
  - Closes on outside click.
  - Closes on `Escape` key press.
  - Smooth slide-in animation.

#### Files

- `src/components/Navbar/Navbar.tsx`: React component logic.
- `src/components/Navbar/Navbar.css`: Component styling (Vanilla CSS).
- `src/components/Navbar/index.ts`: Export entry point.

#### Usage

```tsx
import Navbar from './components/Navbar';

const App = () => {
  return (
    <div>
      <Navbar />
      <main>
        {/* Page content */}
      </main>
    </div>
  );
};
```
