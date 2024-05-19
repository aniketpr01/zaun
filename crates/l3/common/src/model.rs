use starknet_core::types::FieldElement;

#[derive(Drop, Serde)]
struct VectorCommitmentConfigWithSerde {
    height: FieldElement,
    n_verifier_friendly_commitment_layers: FieldElement,
}

#[derive(Drop, Serde)]
struct TableCommitmentConfigWithSerde {
    n_columns: FieldElement,
    vector: VectorCommitmentConfigWithSerde,
}

#[derive(Drop, Serde)]
struct TracesConfigWithSerde {
    original: TableCommitmentConfigWithSerde,
    interaction: TableCommitmentConfigWithSerde,
}

#[derive(Drop, Serde)]
struct TableDecommitmentWithSerde {
    n_values: FieldElement,
    values: Array<FieldElement>,
}

#[derive(Drop, Serde)]
struct TracesDecommitmentWithSerde {
    original: TableDecommitmentWithSerde,
    interaction: TableDecommitmentWithSerde,
}

#[derive(Drop, Serde)]
struct TracesWitnessWithSerde {
    original: TableCommitmentWitnessWithSerde,
    interaction: TableCommitmentWitnessWithSerde,
}

#[derive(Drop, Serde)]
struct VectorCommitmentWitnessWithSerde {
    n_authentications: FieldElement,
    authentications: Array<FieldElement>,
}

#[derive(Drop, Serde)]
struct TableCommitmentWitnessWithSerde {
    vector: VectorCommitmentWitnessWithSerde,
}

#[derive(Drop, Serde)]
struct FriConfigWithSerde {
    log_input_size: FieldElement,
    n_layers: FieldElement,
    inner_layers: Array<FieldElement>,
    fri_step_sizes: Array<FieldElement>,
    log_last_layer_degree_bound: FieldElement,
}

#[derive(Drop, Serde)]
struct ProofOfWorkConfigWithSerde {
    n_bits: FieldElement,
}

#[derive(Drop, Serde)]
struct StarkConfigWithSerde {
    traces: TracesConfigWithSerde,
    composition: TableCommitmentConfigWithSerde,
    fri: FriConfigWithSerde,
    proof_of_work: ProofOfWorkConfigWithSerde,
    log_trace_domain_size: FieldElement,
    n_queries: FieldElement,
    log_n_cosets: FieldElement,
    n_verifier_friendly_commitment_layers: FieldElement,
}

#[derive(Drop, Serde)]
struct PublicInputWithSerde {
    log_n_steps: FieldElement,
    range_check_min: FieldElement,
    range_check_max: FieldElement,
    layout: FieldElement,
    dynamic_params: Array<FieldElement>,
    n_segments: FieldElement,
    segments: Array<FieldElement>,
    padding_addr: FieldElement,
    padding_value: FieldElement,
    main_page_len: FieldElement,
    main_page: Array<FieldElement>,
    n_continuous_pages: FieldElement,
    continuous_page_headers: Array<FieldElement>,
}

#[derive(Drop, Serde)]
struct TracesUnsentCommitmentWithSerde {
    original: FieldElement,
    interaction: FieldElement,
}

#[derive(Drop, Serde)]
struct FriUnsentCommitmentWithSerde {
    inner_layers: Array<FieldElement>,
    last_layer_coefficients: Array<FieldElement>,
}

#[derive(Drop, Serde)]
struct ProofOfWorkUnsentCommitmentWithSerde {
    nonce: FieldElement,
}

#[derive(Drop, Serde)]
struct FriWitnessWithSerde {
    layers: Array<FieldElement>,
}

#[derive(Drop, Serde)]
struct StarkUnsentCommitmentWithSerde {
    traces: TracesUnsentCommitmentWithSerde,
    composition: FieldElement,
    oods_values: Array<FieldElement>,
    fri: FriUnsentCommitmentWithSerde,
    proof_of_work: ProofOfWorkUnsentCommitmentWithSerde,
}

#[derive(Drop, Serde)]
struct StarkWitnessWithSerde {
    traces_decommitment: TracesDecommitmentWithSerde,
    traces_witness: TracesWitnessWithSerde,
    composition_decommitment: TableDecommitmentWithSerde,
    composition_witness: TableCommitmentWitnessWithSerde,
    fri_witness: FriWitnessWithSerde,
}

#[derive(Drop, Serde)]
struct StarkProofWithSerde {
    config: StarkConfigWithSerde,
    public_input: PublicInputWithSerde,
    unsent_commitment: StarkUnsentCommitmentWithSerde,
    witness: StarkWitnessWithSerde,
}