import React, { useMemo } from 'react';
import { Proposal, ProposalStatus } from '../../types/proposal';
import './ProposalDetail.css';

interface ProposalDetailProps {
  proposal: Proposal;
  isAdmin?: boolean;
  onVote?: (type: 'Yes' | 'No' | 'Abstain') => void;
  onFinalize?: () => void;
  onExecute?: () => void;
  onCancel?: () => void;
}

const ProposalDetail: React.FC<ProposalDetailProps> = ({
  proposal,
  isAdmin = false,
  onVote,
  onFinalize,
  onExecute,
  onCancel,
}) => {
  const totalVotes = useMemo(() => {
    return proposal.votesYes + proposal.votesNo + proposal.votesAbstain;
  }, [proposal]);

  const percentages = useMemo(() => {
    if (totalVotes === BigInt(0)) return { yes: 0, no: 0, abstain: 0 };
    return {
      yes: Number((proposal.votesYes * BigInt(100)) / totalVotes),
      no: Number((proposal.votesNo * BigInt(100)) / totalVotes),
      abstain: Number((proposal.votesAbstain * BigInt(100)) / totalVotes),
    };
  }, [proposal, totalVotes]);

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  const isExpired = Date.now() / 1000 > proposal.endTime;

  return (
    <div className="proposal-detail">
      <div className="proposal-header">
        <a href="/proposals" className="back-link">← Back to Proposals</a>
        <div className="proposal-status-badge" data-status={proposal.status}>
          {proposal.status}
        </div>
      </div>

      <h1 className="proposal-title">{proposal.title}</h1>
      
      <div className="proposal-meta">
        <div className="meta-item">
          <span className="meta-label">ID:</span>
          <span className="meta-value">#{proposal.id}</span>
        </div>
        <div className="meta-item">
          <span className="meta-label">Proposer:</span>
          <span className="meta-value truncate">{proposal.proposer}</span>
        </div>
        <div className="meta-item">
          <span className="meta-label">Start Time:</span>
          <span className="meta-value">{formatDate(proposal.startTime)}</span>
        </div>
        <div className="meta-item">
          <span className="meta-label">End Time:</span>
          <span className="meta-value">{formatDate(proposal.endTime)}</span>
        </div>
        <div className="meta-item">
          <span className="meta-label">Quorum:</span>
          <span className="meta-value">{proposal.quorum.toString()} votes</span>
        </div>
      </div>

      <div className="proposal-section">
        <h3>Description</h3>
        <p className="proposal-description">{proposal.description}</p>
      </div>

      <div className="proposal-section">
        <h3>Vote Breakdown</h3>
        <div className="vote-stats">
          <div className="stat-bar-container">
            <div className="stat-bar">
              <div className="bar yes" style={{ width: `${percentages.yes}%` }}></div>
              <div className="bar no" style={{ width: `${percentages.no}%` }}></div>
              <div className="bar abstain" style={{ width: `${percentages.abstain}%` }}></div>
            </div>
          </div>
          <div className="stat-labels">
            <div className="label-item">
              <span className="dot yes"></span>
              <span className="label-text">Yes: {proposal.votesYes.toString()} ({percentages.yes}%)</span>
            </div>
            <div className="label-item">
              <span className="dot no"></span>
              <span className="label-text">No: {proposal.votesNo.toString()} ({percentages.no}%)</span>
            </div>
            <div className="label-item">
              <span className="dot abstain"></span>
              <span className="label-text">Abstain: {proposal.votesAbstain.toString()} ({percentages.abstain}%)</span>
            </div>
          </div>
          <div className="total-votes">
            Total Votes: {totalVotes.toString()} / {proposal.quorum.toString()} (Quorum)
          </div>
        </div>
      </div>

      <div className="proposal-actions">
        {proposal.status === ProposalStatus.Active && !isExpired && (
          <div className="vote-buttons">
            <button onClick={() => onVote?.('Yes')} className="btn btn-yes">Vote Yes</button>
            <button onClick={() => onVote?.('No')} className="btn btn-no">Vote No</button>
            <button onClick={() => onVote?.('Abstain')} className="btn btn-abstain">Abstain</button>
          </div>
        )}

        {proposal.status === ProposalStatus.Active && isExpired && (
          <button onClick={onFinalize} className="btn btn-primary">Finalize Proposal</button>
        )}

        {proposal.status === ProposalStatus.Passed && (
          <button onClick={onExecute} className="btn btn-success" disabled={!isAdmin}>
            {isAdmin ? 'Execute Proposal' : 'Awaiting Execution'}
          </button>
        )}

        {isAdmin && (proposal.status === ProposalStatus.Active || proposal.status === ProposalStatus.Passed) && (
          <button onClick={onCancel} className="btn btn-danger">Cancel Proposal</button>
        )}
      </div>
    </div>
  );
};

export default ProposalDetail;
