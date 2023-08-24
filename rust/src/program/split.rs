// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use super::*;
#[allow(unused_imports)]
use snarkvm::prelude::AleoID;

impl<N: Network> ProgramManager<N> {
    /// Split an Aleo credits record into two separate records. This function does not require a fee.
    ///
    /// @param split_amount The amount of the credit split. This amount will be subtracted from the
    /// value of the record and two new records will be created with the split amount and the remainder
    /// @param amount_record The record to split
    #[allow(clippy::too_many_arguments)]
    pub fn split(
        &mut self,
        amount_record: Record<N, Plaintext<N>>,
        split_amount: u64,
        password: Option<&str>,
    ) -> Result<String> {
        ensure!(
            amount_record.microcredits()? >= split_amount,
            "Credits in amount record must greater than transfer amount specified"
        );

        // Specify the network state query
        let query = Query::from(self.api_client.as_ref().unwrap().base_url());

        // Retrieve the private key.
        let private_key = self.get_private_key(password)?;

        let execution = {
            let rng = &mut rand::thread_rng();

            // Initialize a VM
            let store = ConsensusStore::<N, ConsensusMemory<N>>::open(None)?;
            let vm = VM::from(store)?;

            let inputs = vec![Value::Record(amount_record), Value::from_str(&format!("{}u64", split_amount))?];
            vm.execute(&private_key, ("credits.aleo", "split"), inputs.iter(), None, Some(query), rng)?
        };

        self.broadcast_transaction(execution)
    }
}
