package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class PortalUserView {
    private String id;
    private String login;
    private String name;
    private String role;
    private String email;
    private String actorKind;
    private String clientKind;
    private List<String> permissions;

    public String getId() {
        return this.id;
    }
    
    public void setId(String id) {
        this.id = id;
    }

    public String getLogin() {
        return this.login;
    }
    
    public void setLogin(String login) {
        this.login = login;
    }

    public String getName() {
        return this.name;
    }
    
    public void setName(String name) {
        this.name = name;
    }

    public String getRole() {
        return this.role;
    }
    
    public void setRole(String role) {
        this.role = role;
    }

    public String getEmail() {
        return this.email;
    }
    
    public void setEmail(String email) {
        this.email = email;
    }

    public String getActorKind() {
        return this.actorKind;
    }
    
    public void setActorKind(String actorKind) {
        this.actorKind = actorKind;
    }

    public String getClientKind() {
        return this.clientKind;
    }
    
    public void setClientKind(String clientKind) {
        this.clientKind = clientKind;
    }

    public List<String> getPermissions() {
        return this.permissions;
    }
    
    public void setPermissions(List<String> permissions) {
        this.permissions = permissions;
    }
}
