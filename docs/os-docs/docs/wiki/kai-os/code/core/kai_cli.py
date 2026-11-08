#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
💻 KAI-OS CLI Tool
Command-line Interface für Agenten-Management, Blockchain-Operationen, Governance

Based on KAI-OS Wiki Chapter 12
"""

import argparse
import asyncio
import json
import sys
from typing import Optional
import logging
from datetime import datetime

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class KaiCLI:
    """KAI-OS Command Line Interface"""
    
    def __init__(self):
        self.parser = argparse.ArgumentParser(
            description='KAI-OS CLI v1.0.0-alpha',
            formatter_class=argparse.RawDescriptionHelpFormatter,
            epilog="""
Examples:
  kai agent create --name "DataAnalyzer" --model "llama3-8b-q4"
  kai agent invoke agent_123 --input "Analyze this"
  kai blockchain balance 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  kai governance vote 42 --vote yes --conviction 3
            """
        )
        self._setup_subcommands()
    
    def _setup_subcommands(self):
        """Setup CLI subcommands"""
        subparsers = self.parser.add_subparsers(dest='command', help='Commands')
        
        # Agent Commands
        agent_parser = subparsers.add_parser('agent', help='Agent Management')
        agent_subs = agent_parser.add_subparsers(dest='agent_cmd')
        
        create_agent = agent_subs.add_parser('create', help='Create new agent')
        create_agent.add_argument('--name', required=True, help='Agent name')
        create_agent.add_argument('--model', default='llama3-8b-q4', help='Model to use')
        create_agent.add_argument('--capabilities', nargs='+', default=['read_storage'], help='Agent capabilities')
        
        invoke_agent = agent_subs.add_parser('invoke', help='Invoke agent')
        invoke_agent.add_argument('agent_id', help='Agent ID')
        invoke_agent.add_argument('--input', required=True, help='Input for agent')
        invoke_agent.add_argument('--async', action='store_true', default=True, help='Async mode')
        
        list_agents = agent_subs.add_parser('list', help='List agents')
        list_agents.add_argument('--status', help='Filter by status')
        
        delete_agent = agent_subs.add_parser('delete', help='Delete agent')
        delete_agent.add_argument('agent_id', help='Agent ID to delete')
        
        # Blockchain Commands
        chain_parser = subparsers.add_parser('blockchain', help='Blockchain Operations')
        chain_subs = chain_parser.add_subparsers(dest='chain_cmd')
        
        status = chain_subs.add_parser('status', help='Get blockchain status')
        
        balance = chain_subs.add_parser('balance', help='Get account balance')
        balance.add_argument('address', help='Account address')
        
        transfer = chain_subs.add_parser('transfer', help='Transfer tokens')
        transfer.add_argument('--to', required=True, help='Recipient address')
        transfer.add_argument('--amount', type=int, required=True, help='Amount to transfer')
        transfer.add_argument('--token', default='KAI', help='Token type (KAI or COMPUTE)')
        
        # Governance Commands
        gov_parser = subparsers.add_parser('governance', help='Governance & Voting')
        gov_subs = gov_parser.add_subparsers(dest='gov_cmd')
        
        proposals = gov_subs.add_parser('proposals', help='List proposals')
        
        vote = gov_subs.add_parser('vote', help='Vote on proposal')
        vote.add_argument('proposal_id', type=int, help='Proposal ID')
        vote.add_argument('--vote', required=True, choices=['yes', 'no', 'abstain'], help='Vote')
        vote.add_argument('--conviction', type=int, default=1, help='Conviction level (0-6)')
        
        create_proposal = gov_subs.add_parser('create', help='Create proposal')
        create_proposal.add_argument('--title', required=True, help='Proposal title')
        create_proposal.add_argument('--description', required=True, help='Description')
        create_proposal.add_argument('--target', required=True, help='Target component')
        
        # System Commands
        system_parser = subparsers.add_parser('system', help='System Management')
        system_subs = system_parser.add_subparsers(dest='system_cmd')
        
        info = system_subs.add_parser('info', help='System information')
        status = system_subs.add_parser('status', help='System status')
        version = system_subs.add_parser('version', help='Show version')
    
    async def execute(self, args=None) -> int:
        """Execute CLI command"""
        args = self.parser.parse_args(args)
        
        if not args.command:
            self.parser.print_help()
            return 0
        
        try:
            if args.command == 'agent':
                return await self._handle_agent(args)
            elif args.command == 'blockchain':
                return await self._handle_blockchain(args)
            elif args.command == 'governance':
                return await self._handle_governance(args)
            elif args.command == 'system':
                return await self._handle_system(args)
            else:
                self.parser.print_help()
                return 1
        
        except Exception as e:
            logger.error(f"❌ Error: {str(e)}")
            return 1
    
    async def _handle_agent(self, args) -> int:
        """Handle agent commands"""
        if args.agent_cmd == 'create':
            logger.info(f"🤖 Creating agent: {args.name}")
            print(json.dumps({
                "id": "agent_01HZ...",
                "name": args.name,
                "model": args.model,
                "status": "starting"
            }, indent=2))
            return 0
        
        elif args.agent_cmd == 'invoke':
            logger.info(f"💬 Invoking agent: {args.agent_id}")
            print(json.dumps({
                "task_id": "task_01HZ...",
                "status": "queued" if args.async else "completed"
            }, indent=2))
            return 0
        
        elif args.agent_cmd == 'list':
            logger.info("📋 Listing agents...")
            print(json.dumps({"agents": []}, indent=2))
            return 0
        
        elif args.agent_cmd == 'delete':
            logger.info(f"🗑️ Deleting agent: {args.agent_id}")
            logger.info("✅ Agent deleted")
            return 0
        
        return 1
    
    async def _handle_blockchain(self, args) -> int:
        """Handle blockchain commands"""
        if args.chain_cmd == 'status':
            logger.info("⛓️ Fetching blockchain status...")
            print(json.dumps({
                "block_number": 1048576,
                "peers": 43,
                "sync_status": "synced"
            }, indent=2))
            return 0
        
        elif args.chain_cmd == 'balance':
            logger.info(f"💵 Getting balance for {args.address}")
            print(json.dumps({
                "address": args.address,
                "kai_balance": "1000000000000",
                "compute_balance": "500000"
            }, indent=2))
            return 0
        
        elif args.chain_cmd == 'transfer':
            logger.info(f"🐱 Transferring {args.amount} {args.token} to {args.to}")
            print(json.dumps({
                "tx_hash": "0x1a2b3c...",
                "status": "pending"
            }, indent=2))
            return 0
        
        return 1
    
    async def _handle_governance(self, args) -> int:
        """Handle governance commands"""
        if args.gov_cmd == 'proposals':
            logger.info("📋 Fetching proposals...")
            print(json.dumps({"proposals": []}, indent=2))
            return 0
        
        elif args.gov_cmd == 'vote':
            logger.info(f"🗣️ Voting {args.vote} on proposal #{args.proposal_id}")
            logger.info("✅ Vote recorded")
            return 0
        
        elif args.gov_cmd == 'create':
            logger.info(f"📢 Creating proposal: {args.title}")
            print(json.dumps({
                "proposal_id": 42,
                "title": args.title,
                "status": "discussion"
            }, indent=2))
            return 0
        
        return 1
    
    async def _handle_system(self, args) -> int:
        """Handle system commands"""
        if args.system_cmd == 'info':
            print("""
🧠⛓️ KAI-OS v1.0.0-alpha
AI-Blockchain Operating System

Network: testnet
Version: 1.0.0-alpha
Status: Online
            """)
            return 0
        
        elif args.system_cmd == 'status':
            print(json.dumps({
                "status": "online",
                "version": "1.0.0-alpha",
                "timestamp": datetime.utcnow().isoformat()
            }, indent=2))
            return 0
        
        elif args.system_cmd == 'version':
            print("KAI-OS CLI v1.0.0-alpha")
            return 0
        
        return 1


async def main():
    """Main entry point"""
    cli = KaiCLI()
    exit_code = await cli.execute()
    sys.exit(exit_code)


if __name__ == '__main__':
    asyncio.run(main())
