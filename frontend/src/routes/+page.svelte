<script lang="ts">
	import { AccumulateRequest, AccumulateResponse } from '../generated/model';

	let curr: number | null = $state(null);

	function submit() {
		fetch('http://localhost:3000/accumulate', {
      method: "POST",
			body: AccumulateRequest.encode({ id: '0', number: 1 }).finish()
		})
			.then((res) => res.arrayBuffer())
			.then((res) => AccumulateResponse.decode(new Uint8Array(res)))
      .then(res => void (curr = res.soFar));
	}
</script>


<button onclick={submit}>Accumulate: {curr ?? "nothing"}</button>