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

### Proposal Detail Page

A dedicated page showing all proposal metadata and voting information.

- **Metadata Display**: Shows title, description, proposer (truncated), start/end dates, and quorum.
- **Vote Breakdown**: 
  - Visual progress bar showing the distribution of Yes, No, and Abstain votes.
  - Percentage and absolute count for each vote type.
  - Quorum progress tracking.
- **Contextual Actions**:
  - **Vote**: Available when proposal is Active and not expired.
  - **Finalize**: Available when proposal is Active but expired.
  - **Execute**: Available when proposal is Passed (Admin only).
  - **Cancel**: Available for Active/Passed proposals (Admin only).
- **Navigation**: Includes a "Back to Proposals" link for easy navigation.

#### Files

- `src/types/proposal.ts`: TypeScript interfaces and enums.
- `src/pages/ProposalDetail/ProposalDetail.tsx`: Page component logic.
- `src/pages/ProposalDetail/ProposalDetail.css`: Page styling.

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
