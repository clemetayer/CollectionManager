export type Collection = {
    name: string,
    deezer_id: string,
    url: string,
    tracks : Track[],
    children_col: Collection[]
}

export type Track = {
    deezer_id: string,
    title: string,
    link: string,
    artist: string,
}