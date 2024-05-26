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

export type PostAddCollectionToParent = {
    parent_collection_id : string,
    child_collection_id : string
}