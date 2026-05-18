#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec,
};

/// Struktur data anggota keluarga
#[contracttype]
#[derive(Clone, Debug)]
pub struct FamilyMember {
    pub id: u64,
    pub name: String,

    // Informasi tambahan
    pub birth_date: String,
    pub gender: String,

    // Status hidup
    pub is_alive: bool,
    pub death_date: String,

    // Relasi keluarga
    pub parent_ids: Vec<u64>,
    pub child_ids: Vec<u64>,
}

/// Storage key
const FAMILY_DATA: Symbol = symbol_short!("FAMILY");

#[contract]
pub struct FamilyTreeContract;

#[contractimpl]
impl FamilyTreeContract {

    /// Ambil semua anggota keluarga
    pub fn get_members(env: Env) -> Vec<FamilyMember> {
        env.storage()
            .instance()
            .get(&FAMILY_DATA)
            .unwrap_or(Vec::new(&env))
    }

    /// Tambah anggota keluarga baru
    pub fn create_member(
        env: Env,
        name: String,
        birth_date: String,
        gender: String,
    ) -> String {

        // Ambil data lama
        let mut members: Vec<FamilyMember> = env.storage()
            .instance()
            .get(&FAMILY_DATA)
            .unwrap_or(Vec::new(&env));

        // Buat member baru
        let member = FamilyMember {
            id: env.prng().gen::<u64>(),
            name,
            birth_date,
            gender,

            is_alive: true,
            death_date: String::from_str(&env, ""),

            parent_ids: Vec::new(&env),
            child_ids: Vec::new(&env),
        };

        // Simpan
        members.push_back(member);

        env.storage().instance().set(&FAMILY_DATA, &members);

        String::from_str(&env, "Family member created")
    }

    /// Cari member berdasarkan ID
    pub fn search_by_id(env: Env, id: u64) -> Vec<FamilyMember> {

        let members: Vec<FamilyMember> = env.storage()
            .instance()
            .get(&FAMILY_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for i in 0..members.len() {

            let member = members.get(i).unwrap();

            if member.id == id {
                result.push_back(member);
            }
        }

        result
    }

    /// Cari member berdasarkan nama
    pub fn search_by_name(env: Env, name: String) -> Vec<FamilyMember> {

        let members: Vec<FamilyMember> = env.storage()
            .instance()
            .get(&FAMILY_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for i in 0..members.len() {

            let member = members.get(i).unwrap();

            if member.name == name {
                result.push_back(member);
            }
        }

        result
    }

    /// Tambah relasi parent-child
    ///
    /// parent_id = orang tua
    /// child_id = anak
    pub fn add_parent_child_relation(
        env: Env,
        parent_id: u64,
        child_id: u64,
    ) -> String {

        let mut members: Vec<FamilyMember> = env.storage()
            .instance()
            .get(&FAMILY_DATA)
            .unwrap_or(Vec::new(&env));

        let mut parent_index: u32 = 0;
        let mut child_index: u32 = 0;

        let mut parent_found = false;
        let mut child_found = false;

        // Cari index
        for i in 0..members.len() {

            let member = members.get(i).unwrap();

            if member.id == parent_id {
                parent_index = i;
                parent_found = true;
            }

            if member.id == child_id {
                child_index = i;
                child_found = true;
            }
        }

        if !parent_found || !child_found {
            return String::from_str(&env, "Parent or child not found");
        }

        // Update parent
        let mut parent = members.get(parent_index).unwrap();
        parent.child_ids.push_back(child_id);

        // Update child
        let mut child = members.get(child_index).unwrap();
        child.parent_ids.push_back(parent_id);

        // Simpan kembali
        members.set(parent_index, parent);
        members.set(child_index, child);

        env.storage().instance().set(&FAMILY_DATA, &members);

        String::from_str(&env, "Family relationship added")
    }

    /// Update status meninggal
    pub fn mark_as_deceased(
        env: Env,
        id: u64,
        death_date: String,
    ) -> String {

        let mut members: Vec<FamilyMember> = env.storage()
            .instance()
            .get(&FAMILY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..members.len() {

            let mut member = members.get(i).unwrap();

            if member.id == id {

                member.is_alive = false;
                member.death_date = death_date;

                members.set(i, member);

                env.storage().instance().set(&FAMILY_DATA, &members);

                return String::from_str(
                    &env,
                    "Member marked as deceased"
                );
            }
        }

        String::from_str(&env, "Member not found")
    }

    /// Update status hidup kembali
    pub fn mark_as_alive(env: Env, id: u64) -> String {

        let mut members: Vec<FamilyMember> = env.storage()
            .instance()
            .get(&FAMILY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..members.len() {

            let mut member = members.get(i).unwrap();

            if member.id == id {

                member.is_alive = true;
                member.death_date = String::from_str(&env, "");

                members.set(i, member);

                env.storage().instance().set(&FAMILY_DATA, &members);

                return String::from_str(
                    &env,
                    "Member marked as alive"
                );
            }
        }

        String::from_str(&env, "Member not found")
    }

    /// Hapus member berdasarkan ID
    pub fn remove_member_by_id(env: Env, id: u64) -> String {

        let mut members: Vec<FamilyMember> = env.storage()
            .instance()
            .get(&FAMILY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..members.len() {

            if members.get(i).unwrap().id == id {

                members.remove(i);

                env.storage().instance().set(&FAMILY_DATA, &members);

                return String::from_str(&env, "Member removed");
            }
        }

        String::from_str(&env, "Member not found")
    }

    /// Hapus member berdasarkan nama
    pub fn remove_member_by_name(
        env: Env,
        name: String,
    ) -> String {

        let mut members: Vec<FamilyMember> = env.storage()
            .instance()
            .get(&FAMILY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..members.len() {

            if members.get(i).unwrap().name == name {

                members.remove(i);

                env.storage().instance().set(&FAMILY_DATA, &members);

                return String::from_str(&env, "Member removed");
            }
        }

        String::from_str(&env, "Member not found")
    }
}

mod test;