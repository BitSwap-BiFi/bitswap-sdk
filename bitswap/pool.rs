use crate::types::PeerId;
use crate::types::PeerInfo;
use crate::types::Pool;
use crate::types::PoolInfo;
use crate::types::PoolStats;
use crate::types::PoolAddress;
use crate::types::PoolAddressInfo;
use crate::types::PoolAddressStats;
use crate::types::Pair;
use crate::types::PairInfo;
use crate::types::PairStats;
use crate:::types::Swarm;
use crate::types::SwarmInfo;

fn get_pool_info(pool: &Pool) -> PoolInfo {
    PoolInfo {
        id: pool.id.clone(),
        name: pool.name.clone(),
        description: pool.description.clone(),
        website: pool.website.clone(),
        fee: pool.fee.clone(),
        token_address: pool.token_address.clone(),
        token_id: pool.token_id.clone(),
        token_name: pool.token_name.

        // TODO: add more fields
    }
}

fn get_pool_stats(pool: &Pool) -> PoolStats {
    PoolStats {
        id: pool.id.clone(),
        name: pool.name.clone(),
        description: pool.description.clone(),
        website: pool.website.clone(),
        fee: pool.fee.clone(),
        token_address: pool.token_address.clone(),
        token_id: pool.token_id.clone(),
        token_name: pool.token_name.
    }
}

fn get_pool_address_info(pool_address: &PoolAddress) -> Pool

    AddressInfo {
        id: pool_address.id.clone(),
        name: pool_address.name.clone(),
        description: pool_address.description.clone(),
        website: pool_address.website.clone(),
        fee: pool_address.fee.clone(),
        token_address: pool_address.token_address.clone(),
        token_id: pool_address.token_id.clone(),
        token_name: pool_address.token_name.clone(),

    } 

fn get_pool_address_stats(pool_address: &PoolAddress) -> PoolAddressStats {
    PoolAddressStats {
        id: pool_address.id.clone(),
        name: pool_address.name.clone(),
        description: pool_address.description.clone(),
        website: pool_address.website.clone(),
        fee: pool_address.fee.clone(),
        token_address: pool_address.token_address.clone(),
        token_id

    }
}

fn get_pair_info(pair: &Pair) -> PairInfo {
    PairInfo {
        id: pair.id.clone(),
        name: pair.name.clone
    }
}
fn get_pair_stats(pair: &Pair) -> PairStats {
    PairStats {
        id: pair.id.clone(),
        name: pair.name.clone
    }
}

fn get_swarm_info(swarm: &Swarm) -> SwarmInfo {
    SwarmInfo

    Info {
        id: swarm.id.clone(),
        name: swarm.name.clone(),
        description: swarm.description.clone(),
        website: swarm.website.clone(),
        fee: swarm.fee.clone(),
        token_address: swarm.token_address.clone(),
        token_id: swarm.token_id.clone(),
        token_name: swarm.token_name.clone(),
    }
}

fn get_swarm_stats
    (swarm: &Swarm) -> SwarmStats {
        SwarmStats {
            id: swarm.id.clone(),
            name: swarm.name.clone(),
            description: swarm.description.clone(),
            website: swarm.website.clone(),
            fee: swarm.fee.clone(),
            token_address: swarm.token_address.clone(),
            token_id: swarm.token_id.clone(),
            token_name: swarm.token_name.clone(),
        }
    }

fn get_peer_info(peer: &Peer) -> PeerInfo {
    PeerInfo {
        id: peer.id.clone(),
        name: peer.name.clone(),
        description: peer.description.clone(),
        website: peer.website.clone(),
        fee: peer.fee.clone(),
        token_address: peer.token_address.clone(),
        token_id: peer.token_id.clone(),
        token_name: peer.token_name.clone(),
    }
}

fn get_peer_stats(peer: &Peer) -> PeerStats {
    PeerStats {
        id:
        peer.id.clone(),
        name: peer.name.clone(),
        description: peer.description.clone(),
        website: peer.website.clone(),
        fee: peer.fee.clone(),
        token_address: peer.token_address.clone(),
        token_id: peer.token_id.clone(),
        token_name: peer.token_name.clone(),
    }
}
