// import { create } from "zustand"

// type Data = {
//     speakers: Speaker[]
// }

// type Actions = {
//     set_speakers: (sps: Speaker[]) => void,
//     add_speaker: (sp: Speaker) => void
// }

// export const useSpeakersStore = create<Data & Actions>((set, get) => ({
//     speakers: [],

//     set_speakers(sps) {
//         set({speakers: sps})
//     },
    
//     add_speaker(sp) {
//         const updated_speakers = [...get().speakers, sp]
//         set({speakers: updated_speakers})
//     },
// }))