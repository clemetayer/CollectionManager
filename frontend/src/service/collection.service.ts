import axios from 'axios';
import { ConstantsAPI } from './constantsAPI';
import type { Collection, PostAddCollectionToParent, Track } from '@/models/collection.model';

export class CollectionService {

    private readonly axiosInstance = axios.create({
        baseURL: ConstantsAPI.API_PATH
    });

    async listCollections() : Promise<Collection[]> {
        return this.axiosInstance.get<Collection[]>(ConstantsAPI.PATH_COLLECTION_LIST)
            .then((response) => {
                console.log("response listing collections : " + response);
                return response.data;
            })
            .catch(error => {
                console.error('error listing collections :', error);
                return [];
            })
    }

    async initCollection(name:String, fromPlaylist:String|null) {
        return this.axiosInstance.post(ConstantsAPI.PATH_COLLECTION_INIT, {
                name:name,
                from_playlist:fromPlaylist
            },
            {
                headers: {
                    'Access-Control-Allow-Origin': '*',
                    'Content-Type': 'application/json'
                }
            })
            .catch(error => {
                console.error('error init collection :', error);
            })
    }

    async getCollection(deezer_id: string) : Promise<Collection> {
        return this.axiosInstance.get<Collection>(ConstantsAPI.PATH_COLLECTION + deezer_id)
        .then((response) => {
            console.log("response getting collection : " + response);
            return response.data;
        })
        .catch(error => {
            console.error('error getting collection :', error);
            return { // Default return to not crash the entire page
                name: "",
                deezer_id: deezer_id,
                url: ""
            }
        })
    }

    async getCollectionTracks(deezer_id: string) : Promise<Track[]> {
        return this.axiosInstance.get<Track[]>(ConstantsAPI.PATH_TRACKS + deezer_id)
        .then((response) => {
            console.log("response getting tracks : " + response);
            return response.data;
        })
        .catch(error => {
            console.error('error getting tracks :', error);
            return [] // Default return to not crash the entire page
        })
    }

    async getChildrenCollections(deezer_id: string) : Promise<Collection[]> {
        return this.axiosInstance.get<Collection[]>(ConstantsAPI.PATH_CHILDREN_COLLECTIONS + deezer_id)
        .then((response) => {
            console.log("response getting children collections : " + response);
            return response.data;
        })
        .catch(error => {
            console.error('error getting chidlren collections :', error);
            return [] // Default return to not crash the entire page
        })
    }

    async updateCollection(deezer_id : string) {
        return this.axiosInstance.put(ConstantsAPI.PATH_REFRESH_COLLECTION + deezer_id, undefined,
        {
            headers: {
                'Access-Control-Allow-Origin': '*',
                'Content-Type': 'application/json'
            }
        })
        .catch(error => {
            console.error('error update collection :', error);
        })
    }

    async updateAllCollections() {
        return this.axiosInstance.put(ConstantsAPI.PATH_REFRESH_ALL_COLLECTIONS, undefined,
        {
            headers: {
                'Access-Control-Allow-Origin': '*',
                'Content-Type': 'application/json'
            }
        })
        .catch(error => {
            console.error('error update all collections :', error);
        })
    }

    async addChildCollection(parent_id : string, child_id : string) {
        const body : PostAddCollectionToParent = {
            parent_collection_id: parent_id,
            child_collection_id: child_id
        };
        return this.axiosInstance.post(ConstantsAPI.PATH_ADD_COLLECTION_TO_PARENT, body,
            {
                headers: {
                    'Access-Control-Allow-Origin': '*',
                    'Content-Type': 'application/json'
                }
            })
            .catch(error => {
                console.error('error update all collections :', error);
            })
    }

    async removeChildCollection(parent_id : string, child_id : string) {
        const body : PostAddCollectionToParent = {
            parent_collection_id: parent_id,
            child_collection_id: child_id
        };
        return this.axiosInstance.delete(ConstantsAPI.PATH_REMOVE_COLLECTION_TO_PARENT,
            {
                headers: {
                    'Access-Control-Allow-Origin': '*',
                    'Content-Type': 'application/json'
                },
                data : body
            })
            .catch(error => {
                console.error('error remove child collection :', error);
            })
    }

    async removeCollection(collection_id : string) {

        return this.axiosInstance.delete(ConstantsAPI.PATH_COLLECTION + collection_id,
            {
                headers: {
                    'Access-Control-Allow-Origin': '*',
                    'Content-Type': 'application/json'
                },
            })
            .catch(error => {
                console.error('error remove collection :', error);
            })
    }
}